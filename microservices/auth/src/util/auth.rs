use chrono::{Duration, Utc};

use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use lambda_http::aws_lambda_events::serde::{Deserialize, Serialize};
use microservices::*;

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "lambda_http::aws_lambda_events::serde")]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

pub fn create_jwt(id: &str) -> Result<AuthResponse, Error> {
    lazy_static::lazy_static! {
        static ref JWT_SECRET: EncodingKey = EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes());
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
    let token = encode(&header, &claims, &JWT_SECRET).map_err(|_| Error::TokenCreateFailed)?;
    let refresh =
        encode(&header, &refresh_claims, &JWT_SECRET).map_err(|_| Error::TokenCreateFailed)?;

    Ok(AuthResponse {
        access_token: token,
        refresh_token: refresh,
        expires_in: 60,
    })
}

pub fn verify(token: &str) -> Result<TokenData<Claims>, Error> {
    lazy_static::lazy_static! {
        static ref JWT_SECRET: DecodingKey = DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes());
    };

    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(token, &JWT_SECRET, &validation).map_err(|_| Error::InvalidToken)
}
