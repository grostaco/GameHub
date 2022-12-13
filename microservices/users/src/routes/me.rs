use aws_sdk_dynamodb::{model::AttributeValue, Client};
use lambda_http::{http::Method, Body, Error, Response};
use serde::{Deserialize, Serialize};
use util::{auth::create_jwt, json_response};

#[derive(Serialize, Deserialize)]
struct MeGetReturn {
    username: String,
    bio: String,
    avatar: String,
    friends: Vec<String>,
    games_played: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct MePatchRequest {
    username: Option<String>,
    bio: Option<String>,
    avatar: Option<String>,
}

pub async fn me(
    client: &Client,
    request: &Body,
    method: &Method,
    id: &str,
) -> Result<Response<Body>, Error> {
    Ok(match method {
        &Method::GET => me_get(client, request, id).await?,
        &Method::PATCH => me_patch(client, request, id).await?,
        _ => {
            return Ok(json_response!(
                404,
                "Unknown method",
                format!("{} is not a valid method", method.as_str())
            )?)
        }
    })
}

async fn me_get(client: &Client, _request: &Body, id: &str) -> Result<Response<Body>, Error> {
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
                    &MeGetReturn {
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

async fn me_patch(client: &Client, request: &Body, id: &str) -> Result<Response<Body>, Error> {
    let request: MePatchRequest = match serde_json::from_slice(request) {
        Ok(request) => request,
        Err(e) => return Ok(json_response!(400, "Malformed JSON body", e.to_string())?),
    };

    let mut update = client
        .update_item()
        .table_name("GamehubUsers")
        .key("id", AttributeValue::N(id.into()));
    if let Some(username) = request.username {
        update = update
            .update_expression("SET username = :username")
            .expression_attribute_values(":username", AttributeValue::S(username.into()));
    }
    if let Some(bio) = request.bio {
        update = update
            .update_expression("SET bio = :bio")
            .expression_attribute_values(":bio", AttributeValue::S(bio.into()));
    }
    if let Some(avatar) = request.avatar {
        update = update
            .update_expression("SET avatar = :avatar")
            .expression_attribute_values(":avatar", AttributeValue::S(avatar.into()));
    }

    match update.send().await {
        Ok(_) => return Ok(json_response!(200, &create_jwt(&id).unwrap())?),
        Err(e) => Ok(json_response!(400, "Could not patch user", e.to_string())?),
    }
}
