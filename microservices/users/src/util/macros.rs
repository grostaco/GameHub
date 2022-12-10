use lambda_http::aws_lambda_events::serde::{Deserialize, Serialize};
pub const GAMEHUB_EPOCH: u64 = 1640995200000;

#[derive(Serialize, Deserialize)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
pub struct AuthError {
    pub message: String,
    pub reason: String,
    pub code: u64,
}

macro_rules! json_response {
    ($status:expr, $message:expr, $reason:expr) => {
        Response::builder()
            .status($status)
            .header("content-type", "application/json")
            .body(
                serde_json::to_vec_pretty(&crate::util::macros::AuthError {
                    message: $message.into(),
                    reason: $reason.into(),
                    code: $status,
                })
                .unwrap()
                .into(),
            )
            .map_err(Box::new)
    };

    ($status:expr, $json:expr) => {
        Response::builder()
            .status($status)
            .header("content-type", "application/json")
            .body(serde_json::to_vec_pretty($json).unwrap().into())
            .map_err(Box::new)
    };
}

pub(crate) use json_response;
