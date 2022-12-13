use super::error::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use super::requests::request;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub username: String,
    pub bio: String,
    pub avatar: String,
    pub friends: Vec<String>,
    pub games_played: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPatchRequest {
    pub username: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

pub async fn get_user_info(jwt: &str) -> Result<UserInfoResponse> {
    let mut header_map = HeaderMap::new();
    header_map.insert("Authorization", HeaderValue::from_str(jwt).unwrap());
    request!(get -> "/users/@me" => header_map).await
}

pub async fn patch_user_info(jwt: &str, patch_request: &UserPatchRequest) -> Result<()> {
    let mut header_map = HeaderMap::new();
    header_map.insert("Authorization", HeaderValue::from_str(jwt).unwrap());
    request!(patch -> "/users/@me" ; patch_request => header_map).await?;
    Ok(())
}
