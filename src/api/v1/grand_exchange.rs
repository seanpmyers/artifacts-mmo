use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{ApiEndpoint, HttpRequestMethod},
};

pub const GET_ALL_GE_ITEMS: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/ge/",
    http_request_method: HttpRequestMethod::GET,
};
pub const GET_GE_ITEM: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/ge/{code}",
    http_request_method: HttpRequestMethod::GET,
};
