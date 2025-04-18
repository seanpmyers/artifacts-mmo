use log::{error, warn};
use std::fmt;
use ureq::Body;

use crate::constants::api::ARTIFACTS_MMO_HOST;

pub const HTTP_ACCEPT_HEADER: &str = "accept";
pub const HTTP_AUTHORIZATION_HEADER: &str = "authorization";
pub const HTTP_APPLICATION_JSON: &str = "application/json";

#[derive(Debug)]
pub enum HttpRequestMethod {
    DELETE,
    GET,
    PATCH,
    POST,
    PUT,
}

impl fmt::Display for HttpRequestMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpRequestMethod::DELETE => write!(formatter, "DELETE"),
            HttpRequestMethod::GET => write!(formatter, "GET"),
            HttpRequestMethod::PATCH => write!(formatter, "PATCH"),
            HttpRequestMethod::POST => write!(formatter, "POST"),
            HttpRequestMethod::PUT => write!(formatter, "PUT"),
        }
    }
}

impl HttpRequestMethod {
    pub fn to_ureq_method(&self) -> ureq::http::Method {
        match self {
            HttpRequestMethod::DELETE => ureq::http::Method::DELETE,
            HttpRequestMethod::GET => ureq::http::Method::GET,
            HttpRequestMethod::PATCH => ureq::http::Method::PATCH,
            HttpRequestMethod::POST => ureq::http::Method::POST,
            HttpRequestMethod::PUT => ureq::http::Method::PUT,
        }
    }
}

pub struct ApiEndpoint {
    pub host: &'static str,
    pub path: &'static str,
    pub http_request_method: HttpRequestMethod,
}

impl ApiEndpoint {
    pub fn get_host() -> &'static str {
        ARTIFACTS_MMO_HOST
    }
}

pub fn get_headers(api_token: &String) -> Vec<(String, String)> {
    vec![
        (
            HTTP_ACCEPT_HEADER.to_string(),
            HTTP_APPLICATION_JSON.to_string(),
        ),
        (
            HTTP_AUTHORIZATION_HEADER.to_string(),
            format!("Bearer {}", api_token),
        ),
    ]
}

pub fn make_api_call<T: serde::Serialize + ureq::AsSendBody + for<'de> serde::Deserialize<'de>>(
    http_client: &mut ureq::Agent,
    headers: Vec<(String, String)>,
    http_request_method: HttpRequestMethod,
    uri: ureq::http::Uri,
    body: Option<T>,
) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
    let mut request_builder: ureq::http::request::Builder = ureq::http::Request::builder()
        .uri(uri)
        .method(http_request_method.to_ureq_method());

    for header in headers {
        request_builder = request_builder.header(&header.0, &header.1);
    }

    match body {
        Some(body) => Ok(http_client.run(request_builder.body(body)?)?),
        None => Ok(http_client.run(request_builder.body(())?)?),
    }
}
