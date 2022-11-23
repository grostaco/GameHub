use lambda_http::{
    aws_lambda_events::{
        serde::{Deserialize, Serialize},
        serde_json,
    },
    Body, Error, Response,
};
use microservices::*;

use crate::util::auth::{self, create_jwt};

use crate::util::macros::json_response;

#[derive(Serialize, Deserialize)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
struct RefreshRequest {
    refresh_token: String,
}

pub async fn refresh(request: &Body) -> Result<Response<Body>, Error> {
    let request: RefreshRequest = match serde_json::from_slice(request) {
        Ok(request) => request,
        Err(e) => return Ok(json_response!(400, "Malformed JSON body", e.to_string())?),
    };

    Ok(match auth::verify(&request.refresh_token) {
        Ok(auth) => json_response!(200, &create_jwt(&auth.claims.sub.to_string()).unwrap())?,
        Err(e) => json_response!(401, "Refresh error", e.to_string())?,
    })
}
