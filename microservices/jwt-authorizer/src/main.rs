use lambda_http::{
    aws_lambda_events::serde_json, run, service_fn, Body, Error, Request, RequestExt, Response,
};
use 
    lambda_http::aws_lambda_events::apigw::{
        ApiGatewayCustomAuthorizerPolicy, ApiGatewayCustomAuthorizerRequest,
    };

async fn function_handler(event: Request) -> Result<Response<Body>, lambda_runtime::Error> {
    Ok(Response::default())
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
