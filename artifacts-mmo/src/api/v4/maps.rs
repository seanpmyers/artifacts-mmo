use crate::api::{Endpoint, PAGE_SIZE_DEFAULT};

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetMapRequest {
    x: i32,
    y: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetMapResponse {
    data: Map,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAllMapsRequest {
    pub content_code: Option<String>,
    pub content_type: Option<MapContentType>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAllMapsResponse {
    pub data: Vec<Map>,
    pub total: u32,
    pub page: u32,
    pub size: u32,
    pub pages: u32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Map {
    pub content: Option<MapContent>,
    pub name: String,
    pub skin: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct MapContent {
    pub code: String,
    #[serde(rename = "type")]
    pub content_type: MapContentType,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MapContentType {
    #[default]
    Bank,
    GrandExchange,
    Monster,
    Npc,
    Resource,
    TasksMaster,
    Workshop,
}

impl std::fmt::Display for MapContentType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                MapContentType::Bank => "bank",
                MapContentType::GrandExchange => "grand_exchange",
                MapContentType::Monster => "monster",
                MapContentType::Npc => "npc",
                MapContentType::Resource => "resource",
                MapContentType::TasksMaster => "tasks_master",
                MapContentType::Workshop => "workshop",
            }
        )
    }
}

impl Endpoint for GetMapRequest {
    type Response = GetMapResponse;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn path(&self) -> String {
        format!("/maps/{}/{}", self.x, self.y)
    }
}

impl Endpoint for GetAllMapsRequest {
    type Response = GetAllMapsResponse;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn pageable() -> bool {
        true
    }

    fn path(&self) -> String {
        "/maps".to_string()
    }

    fn query(&self) -> Option<String> {
        let page_query: String = format!(
            "&size={}&page={}",
            match self.page_size {
                Some(size) => size,
                None => Self::page_size(),
            },
            self.page.unwrap_or(PAGE_SIZE_DEFAULT),
        );
        match (&self.content_code, &self.content_type) {
            (None, None) => Some(page_query),
            (None, Some(content_type)) => Some(format!(
                "content_type={}{}",
                content_type.to_string(),
                page_query
            )),
            (Some(content_code), None) => {
                Some(format!("content_code={}{}", content_code, page_query))
            }
            (Some(content_code), Some(content_type)) => Some(format!(
                "content_code={}&content_type={}{}",
                content_code, content_type, page_query
            )),
        }
    }
}
