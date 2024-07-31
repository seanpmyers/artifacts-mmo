pub mod handler;

use serde::{Deserialize, Serialize};

use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{ApiEndpoint, HttpRequestMethod},
};

use super::grand_exchange::GeItemSchema;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetItemRequest {
    pub code: String,
}

impl GetItemRequest {
    pub fn get_path(code: String) -> String {
        GET_ITEM.path.replace("{code}", &code)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetItemResponse {
    pub data: SingleItemSchema,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SingleItemSchema {
    pub item: ItemSchema,
    pub ge: Option<GeItemSchema>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemSchema {
    pub name: String,
    pub code: String,
    pub level: i32,
    #[serde(rename = "type")]
    pub item_type: String,
    pub subtype: String,
    pub description: String,
    pub effects: Vec<ItemEffectSchema>,
    pub craft: Option<CraftSchema>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemEffectSchema {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CraftSchema {
    pub skill: Option<String>,
    pub level: Option<i32>,
    pub items: Vec<SimpleItemSchema>,
    pub quantity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleItemSchema {
    pub code: String,
    pub quantity: i32,
}
