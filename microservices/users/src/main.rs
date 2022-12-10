use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use routes::me;
use util::{auth::verify, json_response};

mod routes;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let query_map = event.path_parameters();
    let proxy_path = query_map
        .iter()
        .find_map(|q| if q.0 == "proxy" { Some(q.1) } else { None })
        .unwrap_or("");

    let id = match event.headers().get("authorization") {
        Some(auth) => match verify(auth.to_str()?) {
            Ok(verified) => verified.claims.sub,
            Err(_) => return Ok(json_response!(401, "Unauthorized", "The JWT is not valid")?),
        },
        None => {
            return Ok(json_response!(
                404,
                format!("Cannot proxy to path '{proxy_path}'"),
                "Path is not recognized"
            )?)
        }
    };

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let body = event.body();

    Ok(match proxy_path {
        "@me" => me::me(&client, body, event.method(), &id).await?,
        _ => json_response!(
            404,
            format!("Cannot proxy to path '{proxy_path}'"),
            "Path is not recognized"
        )?,
    })
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
