use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{ApiEndpoint, HttpRequestMethod},
};

pub const GET_ALL_ITEMS: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/items",
    http_request_method: HttpRequestMethod::GET,
};
pub const GET_ITEM: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/items/{code}",
    http_request_method: HttpRequestMethod::GET,
};
