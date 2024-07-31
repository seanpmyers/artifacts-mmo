use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{ApiEndpoint, HttpRequestMethod},
};

pub const GET_ALL_MAPS: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/maps",
    http_request_method: HttpRequestMethod::GET,
};
pub const GET_MAP: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/maps/{code}",
    http_request_method: HttpRequestMethod::GET,
};
