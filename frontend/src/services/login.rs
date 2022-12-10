use super::error::Result;
use super::requests::request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest<'s> {
    username: &'s str,
    password: &'s str,
}

pub async fn login(username: &str, password: &str) -> Result<LoginResponse> {
    request!(post -> "/auth/login" ; LoginRequest { username, password }).await
}
