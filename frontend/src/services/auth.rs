use super::error::Result;
use super::requests::request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest<'s> {
    username: &'s str,
    password: &'s str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest<'s> {
    username: &'s str,
    password: &'s str,
    email: &'s str,
}

pub async fn login(username: &str, password: &str) -> Result<AuthResponse> {
    request!(post -> "/auth/login" ; LoginRequest { username, password }).await
}

pub async fn register(username: &str, password: &str, email: &str) -> Result<AuthResponse> {
    request!(post -> "/auth/register" ; RegisterRequest { username, password, email }).await
}
