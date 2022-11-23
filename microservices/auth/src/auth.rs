use chrono::{Duration, Utc};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use lambda_http::aws_lambda_events::serde::{Deserialize, Serialize};
use microservices::*;

use super::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
pub struct AuthResponse {
    pub token: String,
    pub refresh: String,
    pub expires_in: u64,
}

pub fn create_jwt(id: &str) -> Result<AuthResponse, Error> {
    lazy_static::lazy_static! {
        static ref JWT_SECRET: String = std::env::var("JWT_SECRET").unwrap();
    };

    // For testing purposes, the tokens will expire very quickly

    let token_expiration = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("Duration add should not fail")
        .timestamp();

    let refresh_expiration = Utc::now()
        .checked_add_signed(Duration::seconds(360))
        .expect("Duration add should not fail")
        .timestamp();

    let claims = Claims {
        sub: id.to_string(),
        exp: usize::try_from(token_expiration).expect("Timestamp should not be negative"),
    };

    let refresh_claims = Claims {
        sub: id.to_string(),
        exp: usize::try_from(refresh_expiration).expect("Timestamp should not be negative"),
    };

    let header = Header::new(Algorithm::HS256);
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|_| Error::TokenCreateFailed)?;
    let refresh = encode(
        &header,
        &refresh_claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|_| Error::TokenCreateFailed)?;

    Ok(AuthResponse {
        token,
        refresh,
        expires_in: 60,
    })
}
