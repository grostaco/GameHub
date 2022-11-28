use aws_sdk_dynamodb::Client;

use lambda_http::{
    aws_lambda_events::serde_json, run, service_fn, Body, Error, Request, RequestExt, Response,
};

pub mod error;
pub mod routes;
pub mod util;

pub use routes::{login, refresh, register};
use util::macros::json_response;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let query_map = event.path_parameters();
    let proxy_path = query_map
        .iter()
        .find_map(|q| if q.0 == "proxy" { Some(q.1) } else { None })
        .unwrap_or("");

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let resp = match proxy_path {
        "login" => login(&client, event.body()).await?,
        "register" => register(&client, event.body()).await?,
        "refresh" => refresh(event.body()).await?,
        _ => json_response!(
            404,
            format!("Cannot proxy to path '{proxy_path}'"),
            "Path is not recognized"
        )?,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    tracing::info!("Running function handler");
    run(service_fn(function_handler)).await
}
