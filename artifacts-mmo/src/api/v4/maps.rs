use crate::api::{Endpoint, EndpointResponse};

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
    pub page_size: Option<i32>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAllMapsResponse {
    pub data: Vec<Map>,
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

    fn call(
        &mut self,
        bearer_token: String,
        http_client: &mut ureq::Agent,
    ) -> EndpointResponse<GetMapResponse> {
        todo!()
    }

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn pageable() -> bool {
        false
    }

    fn path(&self) -> String {
        format!("/maps/{}/{}", self.x, self.y)
    }

    fn query(&self) -> Option<String> {
        None
    }

    fn request_body(&self) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
}

impl Endpoint for GetAllMapsRequest {
    type Response = GetAllMapsResponse;

    fn call(
        &mut self,
        bearer_token: String,
        http_client: &mut ureq::Agent,
    ) -> EndpointResponse<GetAllMapsResponse> {
        todo!()
    }

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
            "&size={}",
            match self.page_size {
                Some(size) => size,
                None => Self::default_page_size(),
            }
        );
        match (&self.content_code, &self.content_type) {
            (None, None) => None,
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

    fn request_body(&self) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
}
