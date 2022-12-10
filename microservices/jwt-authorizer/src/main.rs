// use lambda_http::{
//     aws_lambda_events::apigw::ApiGatewayCustomAuthorizerRequest, run, service_fn, Body, Context,
//     Error, Request, RequestExt, Response,
// };
// use serde::Serialize;
// use tracing::info;

// const POLICY_VERSION: &str = "2012-10-17";

// #[derive(Serialize)]
// pub struct SimpleResponse {
//     #[serde(rename = "isAuthorized")]
//     isAuthorized: bool,
// }

// async fn function_handler(request: Request) -> Result<Response<Body>, lambda_runtime::Error> {
//     let authorization = request.headers().get("authorization");

//     info!(?request, "Request parsed");

//     Ok(Response::builder()
//         .header("Content-Type", "application/json")
//         .body(
//             serde_json::to_string(&SimpleResponse { isAuthorized: true })
//                 .unwrap()
//                 .into(),
//         )
//         .unwrap())
// }

use lambda_http::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    tracing::info!("Running function handler");
    //run(service_fn(function_handler)).await?;

    Ok(())
}
