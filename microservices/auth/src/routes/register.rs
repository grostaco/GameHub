use std::{
    time::{Duration, SystemTime, UNIX_EPOCH},
    vec,
};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use bcrypt::DEFAULT_COST;
use lambda_http::{
    aws_lambda_events::{
        serde::{Deserialize, Serialize},
        serde_json,
    },
    Body, Error, Response,
};

use util::{auth::create_jwt, json_response, macros::GAMEHUB_EPOCH};

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
        .item("username", username_av.clone())
        .item("id", id_av.clone())
        .item("hashed_password", password_av)
        .item("email", email_av)
        .send()
        .await
    {
        Ok(_) => {
            client
                .put_item()
                .table_name("GamehubUsers")
                .item("id", id_av)
                .item("username", username_av)
                .item("bio", AttributeValue::S(String::new()))
                .item("avatar", AttributeValue::S("https://scontent.fbkk5-6.fna.fbcdn.net/v/t39.30808-1/300618868_382834244010803_6059222766905926893_n.png?stp=c4.0.200.200a_dst-png_p200x200&_nc_cat=102&ccb=1-7&_nc_sid=c6021c&_nc_eui2=AeGui9mRiDYAxVG5XH716DlnU6AiVfKaB8RToCJV8poHxNh7yH78Ctr5EejaoGMXdkSs5IRJJzKfcxf6SjUOd0R5&_nc_ohc=EgsGnRuC24YAX87vOk5&_nc_ht=scontent.fbkk5-6.fna&oh=00_AfA-kg8nUPc0P_7kAw034ocEu7NuX_4w1aFYenGQ-xcLYw&oe=639DC7F4".into()))
                .item("friends", AttributeValue::Ns(vec!["0".into()]))
                .item("games_played", AttributeValue::Ns(vec!["0".into()]))
                .send()
                .await
                .unwrap();

            return Ok(json_response!(200, &create_jwt(&id).unwrap())?);
        }
        Err(_) => Ok(json_response!(
            400,
            "Could not add user",
            "Username or password exists"
        )?),
    }
}
