use lambda_http::{run, service_fn, Body, Error, Request, Response};

mod routes;

async fn function_handler(request: Request) -> Result<Response<Body>, lambda_runtime::Error> {
    Ok(Response::builder().body("Hello there!".into()).unwrap())
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
