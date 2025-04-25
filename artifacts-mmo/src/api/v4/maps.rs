use crate::api::{Endpoint, PageInput, PageOutput};

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
    #[serde(flatten)]
    pub page_input: PageInput,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAllMapsResponse {
    pub data: Vec<Map>,
    #[serde(flatten)]
    pub page_output: PageOutput,
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

    fn query_parameters(&self) -> Vec<(String, String)> {
        let mut parameters = self.page_input.to_tuples();

        if let Some(content_code) = &self.content_code {
            parameters.push((String::from("content_code"), content_code.clone()));
        }
        if let Some(content_type) = &self.content_type {
            parameters.push((String::from("content_type"), content_type.to_string()));
        }

        parameters
    }
}
