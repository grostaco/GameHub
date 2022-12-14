use aws_sdk_dynamodb::Client;
use lambda_http::{
    http::{HeaderValue, Method},
    run, service_fn, Body, Error, Request, RequestExt, Response,
};
use routes::me;
use tracing::info;
use util::{auth::verify, json_response};

use crate::routes::util::get_user;

mod routes;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // CORS
    if event.method() == Method::OPTIONS {
        return Ok(Response::builder()
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, PATCH")
            .header(
                "Access-Control-Allow-Headers",
                "Content-Type, Authorization",
            )
            .body("".into())
            .map_err(Box::new)?);
    }

    let query_map = event.path_parameters();
    let proxy_path = query_map
        .iter()
        .find_map(|q| if q.0 == "proxy" { Some(q.1) } else { None })
        .unwrap_or("");

    let id = match event.headers().get("authorization") {
        Some(auth) => match verify(auth.to_str()?.trim()) {
            Ok(verified) => verified.claims.sub,
            Err(e) => return Ok(json_response!(401, "Unauthorized", e.to_string())?),
        },
        None => {
            return Ok(json_response!(
                401,
                "Unauthorized",
                "Authorization not provided"
            )?)
        }
    };

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let body = event.body();

    info!(proxy_path, "Received proxy path");
    let mut resp = match proxy_path {
        "@me" => me::me(&client, body, event.method(), &id).await?,
        id if proxy_path.parse::<u64>().is_ok() => get_user(&client, body, id).await?,
        _ => json_response!(
            404,
            format!("Cannot proxy to path '{proxy_path}'"),
            "Path is not recognized"
        )?,
    };

    let headers = resp.headers_mut();

    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("*"),
    );

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
    run(service_fn(function_handler)).await?;

    Ok(())
}
