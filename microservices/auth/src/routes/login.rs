use aws_sdk_dynamodb::{model::AttributeValue, Client};
use lambda_http::{
    aws_lambda_events::{
        serde::{Deserialize, Serialize},
        serde_json,
    },
    Body, Error, Response,
};
use util::{auth::create_jwt, json_response};

#[derive(Serialize, Deserialize)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login(client: &Client, request: &Body) -> Result<Response<Body>, Error> {
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
                let id = item.get("id").unwrap().as_n().unwrap();

                if bcrypt::verify(request.password, &hashed_password).unwrap_or(false) {
                    return Ok(json_response!(200, &create_jwt(id).unwrap())?);
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
