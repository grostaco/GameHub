use std::time::{Duration, SystemTime, UNIX_EPOCH};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use bcrypt::DEFAULT_COST;
use lambda_http::{
    aws_lambda_events::{
        serde::{Deserialize, Serialize},
        serde_json,
    },
    Body, Error, Response,
};
use microservices::{aws_sdk_dynamodb, lambda_http};

use crate::util::{auth::create_jwt, macros::json_response, macros::GAMEHUB_EPOCH};

#[derive(Serialize, Deserialize)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
struct RegisterRequest {
    username: String,
    password: String,
    email: String,
}

pub async fn register(client: &Client, request: &Body) -> Result<Response<Body>, Error> {
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
    let id = ((elapsed_dur << 22) | 0x454c46).to_string();

    let id_av = AttributeValue::N(id.clone());
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
        Ok(_) => return Ok(json_response!(200, &create_jwt(&id).unwrap())?),
        Err(_) => Ok(json_response!(
            400,
            "Could not add user",
            "Username or password exists"
        )?),
    }
}
