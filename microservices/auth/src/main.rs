use std::time::{Duration, SystemTime, UNIX_EPOCH};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use bcrypt::DEFAULT_COST;
use lambda_http::{
    aws_lambda_events::{
        serde::{Deserialize, Serialize},
        serde_json,
    },
    run, service_fn, Body, Error, Request, RequestExt, Response,
};

const GAMEHUB_EPOCH: u64 = 1640995200000;

macro_rules! json_response {
    ($status:expr, $message:expr, $reason:expr) => {
        Response::builder()
            .status($status)
            .header("content-type", "application/json")
            .body(
                serde_json::to_vec_pretty(&AuthError {
                    message: $message.into(),
                    reason: $reason.into(),
                    code: $status,
                })
                .unwrap()
                .into(),
            )
            .map_err(Box::new)
    };
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
struct RegisterRequest {
    username: String,
    password: String,
    email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
pub struct User {
    id: u64,
    username: String,
    email: String,
    hashed_password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
struct AuthError {
    message: String,
    reason: String,
    code: u64,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let query_map = event.path_parameters();
    let proxy_path = query_map
        .iter()
        .find_map(|q| if q.0 == "proxy" { Some(q.1) } else { None })
        .unwrap_or("");

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let resp = match proxy_path {
        "login" => login(&client, event.body()).await?,
        "register" => register(&client, event.body()).await?,
        _ => json_response!(
            404,
            format!("Cannot proxy to path '{proxy_path}'"),
            "Path is not recognized"
        )?,
    };

    Ok(resp)
}

async fn register(client: &Client, request: &Body) -> Result<Response<Body>, Error> {
    let request: RegisterRequest = match serde_json::from_slice(request) {
        Ok(request) => request,
        Err(e) => return Ok(json_response!(400, "Malformed JSON body", e.to_string())?),
    };

    let elapsed_dur = u64::try_from(
        (SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
            - Duration::from_millis(GAMEHUB_EPOCH))
        .as_millis()
            & 0x3ffffffffff,
    )
    .unwrap();

    let id_av = AttributeValue::N(((elapsed_dur << 22) | 0x454c46).to_string());
    let username_av = AttributeValue::S(request.username);
    let password_av = AttributeValue::S(bcrypt::hash(request.password, DEFAULT_COST).unwrap());
    let email_av = AttributeValue::S(request.email);

    match client
        .put_item()
        .condition_expression("attribute_not_exists(username) OR attribute_not_exists(email)")
        .table_name("GamehubAuth")
        .item("username", username_av)
        .item("id", id_av)
        .item("hashed_password", password_av)
        .item("email", email_av)
        .send()
        .await
    {
        Ok(_) => Ok(json_response!(200, "Success", "User successfully added")?),
        Err(_) => Ok(json_response!(
            400,
            "Could not add user",
            "Username or password exists"
        )?),
    }
}

async fn login(client: &Client, request: &Body) -> Result<Response<Body>, Error> {
    let request: LoginRequest = match serde_json::from_slice(request) {
        Ok(request) => request,
        Err(e) => return Ok(json_response!(400, "Malformed JSON body", e.to_string())?),
    };
    let results = client
        .query()
        .table_name("GamehubAuth")
        .key_condition_expression("#username = :name")
        .expression_attribute_names("#username", "username")
        .expression_attribute_values(":name", AttributeValue::S(request.username.clone()))
        .send()
        .await?;

    if let Some(items) = results.items() {
        match items.first() {
            Some(item) => {
                let hashed_password = item
                    .get("hashed_password")
                    .map(|v| v.as_s().unwrap())
                    .unwrap();

                if bcrypt::verify(request.password, &hashed_password).unwrap_or(false) {
                    return Ok(json_response!(200, "Authorized", "User is authorized")?);
                }
                return Ok(json_response!(
                    401,
                    "Unauthorized",
                    "Password does not match"
                )?);
            }
            None => {
                tracing::info!(
                    username = &request.username,
                    "Cannot find user in dynamoDB. DynamoDB not empty"
                );
                return Ok(json_response!(
                    400,
                    "User not found",
                    "User cannot be found in dynamoDB"
                )?);
            }
        }
    } else {
        tracing::info!(
            username = &request.username,
            "Cannot find user. Unclear how this was arrived at"
        );
        Ok(Response::default())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    tracing::info!("Running function handler");
    run(service_fn(function_handler)).await
}
