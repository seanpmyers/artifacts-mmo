use std::fmt;

use crate::constants::api::ARTIFACTS_MMO_HOST;

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
