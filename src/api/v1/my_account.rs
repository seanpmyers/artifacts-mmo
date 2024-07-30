use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{ApiEndpoint, HttpRequestMethod},
};

pub const BANK_ITEMS: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/my/bank/items",
    http_request_method: HttpRequestMethod::GET,
};
pub const BANK_GOLD: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/my/bank/gold",
    http_request_method: HttpRequestMethod::GET,
};
pub const CHANGE_PASSWORD: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/my/change_password",
    http_request_method: HttpRequestMethod::POST,
};
