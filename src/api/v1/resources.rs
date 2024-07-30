use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{ApiEndpoint, HttpRequestMethod},
};

pub const GET_ALL_RESOURCES: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/resources/",
    http_request_method: HttpRequestMethod::GET,
};
pub const GET_RESOURCES: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/resources/{code}",
    http_request_method: HttpRequestMethod::GET,
};
