use std::fmt::Debug;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Method, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};

use super::error::{Error, ErrorInfo};

macro_rules! request {
    (get -> $url:expr) => {
        crate::services::requests::request_impl(
            reqwest::Method::GET,
            $url,
            (),
            reqwest::header::HeaderMap::default(),
        )
    };

    (get -> $url:expr => $headermap:expr) => {
        crate::services::requests::request_impl(reqwest::Method::GET, $url, (), $headermap)
    };

    (delete -> $url:expr) => {
        crate::services::requests::request_impl(
            reqwest::Method::DELETE,
            $url,
            (),
            reqwest::header::HeaderMap::default(),
        )
    };

    (post -> $url:expr ; $body:expr) => {
        crate::services::requests::request_impl(
            reqwest::Method::POST,
            $url,
            $body,
            reqwest::header::HeaderMap::default(),
        )
    };

    (patch -> $url:expr ; $body:expr => $headers:expr) => {
        crate::services::requests::request_impl(reqwest::Method::PATCH, $url, $body, $headers)
    };
}

pub(crate) use request;

pub async fn request_impl<B, T>(
    method: Method,
    url: &str,
    body: B,
    headers: HeaderMap<HeaderValue>,
) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + Debug,
    B: Serialize + Debug,
{
    let allow_body = matches!(method, Method::POST | Method::PUT | Method::PATCH);
    let baseurl = "https://4ube83bvgl.execute-api.us-east-1.amazonaws.com";
    let url = format!("{baseurl}{url}");

    let mut builder = Client::new()
        .request(method, url)
        .header("Content-Type", "application/json")
        .headers(headers);

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        match data.status() {
            StatusCode::OK => {
                let data = data.json::<T>().await;
                match data {
                    Ok(data) => Ok(data),
                    Err(_) => Err(Error::DeserializeError),
                }
            }
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::FORBIDDEN => Err(Error::Forbidden),
            StatusCode::NOT_FOUND => Err(Error::NotFound),
            StatusCode::INTERNAL_SERVER_ERROR => Err(Error::InternalServerError),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                match data {
                    Ok(data) => Err(Error::UnprocessableEntity(data)),
                    Err(_) => Err(Error::DeserializeError),
                }
            }
            code => {
                log::error!("Log error: {code:#?}");
                Err(Error::RequestError(code))
            }
        }
    } else {
        Err(Error::RequestError(StatusCode::NOT_FOUND))
    }
}
