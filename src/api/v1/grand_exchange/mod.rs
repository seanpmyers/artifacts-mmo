use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeItemSchema {
    pub code: String,
    pub stock: i32,
    pub sell_price: Option<i32>,
    pub buy_price: Option<i32>,
}
