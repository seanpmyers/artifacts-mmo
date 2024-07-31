use log::{error, warn};
use std::fmt;
use url::Url;

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

pub fn make_api_call<T: serde::Serialize>(
    http_client: &mut ureq::Agent,
    headers: Vec<(String, String)>,
    http_request_method: HttpRequestMethod,
    url: Url,
    body: Option<T>,
) -> Option<ureq::Response> {
    let mut request: ureq::Request =
        http_client.request_url(&http_request_method.to_string(), &url);
    for header in headers {
        request = request.set(&header.0, &header.1);
    }
    match body {
        Some(body) => {
            let result = request.send_json(body);
            match result {
                Ok(response) => return Some(response),
                Err(error) => match error {
                    ureq::Error::Status(code, response) => match code {
                        404 => error!("response: {:?}", response),
                        486 => warn!("{:?} ", response),
                        493 => error!("response: {:?}", response),
                        498 => error!("response: {:?}", response),
                        499 => warn!("response: {:?}", response),
                        _ => error!(" response: {:?}", response),
                    },
                    ureq::Error::Transport(_) => error!("{:?}", error),
                },
            };
            None
        }
        None => {
            let result = request.call();
            match result {
                Ok(response) => return Some(response),
                Err(error) => match error {
                    ureq::Error::Status(code, response) => match code {
                        404 => error!("response: {:?}", response),
                        486 => warn!("{:?} ", response),
                        493 => error!("response: {:?}", response),
                        498 => error!("response: {:?}", response),
                        499 => warn!("response: {:?}", response),
                        _ => error!(" response: {:?}", response),
                    },
                    ureq::Error::Transport(_) => error!("{:?}", error),
                },
            };
            None
        }
    }
}
