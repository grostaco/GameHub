use aws_sdk_dynamodb::{model::AttributeValue, Client};
use lambda_http::{Body, Error, Response};
use serde::{Deserialize, Serialize};
use util::json_response;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub bio: String,
    pub avatar: String,
    pub friends: Vec<String>,
    pub games_played: Vec<String>,
}

pub async fn get_user(client: &Client, _request: &Body, id: &str) -> Result<Response<Body>, Error> {
    let results = client
        .query()
        .table_name("GamehubUsers")
        .key_condition_expression("#id = :id")
        .expression_attribute_names("#id", "id")
        .expression_attribute_values(":id", AttributeValue::N(id.into()))
        .send()
        .await?;

    if let Some(items) = results.items() {
        match items.first() {
            Some(item) => {
                let username = item
                    .get("username")
                    .expect("expected username")
                    .as_s()
                    .unwrap()
                    .into();
                let bio = item
                    .get("bio")
                    .expect("expected bio")
                    .as_s()
                    .unwrap()
                    .into();
                let avatar = item
                    .get("avatar")
                    .map(|avatar| avatar.as_s().unwrap().to_string())
                    .unwrap_or("".to_string());
                let friends = item
                    .get("friends")
                    .map(|friends| friends.as_ns().unwrap().clone())
                    .unwrap_or(Vec::new());
                let games_played = item
                    .get("games_played")
                    .map(|games| games.as_ns().unwrap().clone())
                    .unwrap_or(Vec::new());
                return Ok(json_response!(
                    200,
                    &UserResponse {
                        id: id.to_string(),
                        username,
                        bio,
                        avatar,
                        friends,
                        games_played
                    }
                )?);
            }
            None => unimplemented!(),
        }
    }

    unimplemented!()
}
