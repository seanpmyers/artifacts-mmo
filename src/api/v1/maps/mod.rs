pub mod handler;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllMapsResponse {
    pub data: Vec<Map>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Map {
    pub content: Option<MapContent>,
    pub name: String,
    pub skin: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MapContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub code: String,
}
