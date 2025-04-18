pub mod accounts {
    use crate::api::Endpoint;

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct CreateAccountRequest {
        pub email: Option<String>,
        pub password: String,
        pub username: String,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct CreateAccountResponse {}

    impl Endpoint for CreateAccountRequest {
        fn http_request_method() -> http::Method {
            http::Method::POST
        }

        fn pageable() -> bool {
            false
        }

        fn path(&self) -> String {
            "/accounts/create".to_string()
        }

        fn query(&self) -> Option<String> {
            None
        }

        type Response = CreateAccountResponse;

        fn request_body(&self) -> Option<CreateAccountRequest> {
            Some(self.clone())
        }
    }
}

pub mod my_account {}
pub mod my_characters {}
pub mod status {
    use crate::api::Endpoint;

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct StatusRequest {}

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct StatusResponse {
        data: GameStatus,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct GameStatus {
        pub announcements: Option<Vec<Announcements>>,
        pub characters_online: i32,
        pub last_wipe: String,
        pub max_level: i32,
        pub next_wipe: String,
        #[serde(with = "chrono::serde::ts_seconds_option")]
        pub server_time: Option<chrono::DateTime<chrono::Utc>>,
        pub status: String,
        pub version: String,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct Announcements {
        pub message: String,
        #[serde(with = "chrono::serde::ts_seconds_option")]
        pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub enum ServerStatus {
        Offline,
        Online,
        #[default]
        Unknown,
    }

    impl std::fmt::Display for ServerStatus {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ServerStatus::Online => write!(formatter, "Online"),
                ServerStatus::Unknown => write!(formatter, "Unknown"),
                ServerStatus::Offline => write!(formatter, "Offline"),
            }
        }
    }

    impl Endpoint for StatusRequest {
        type Response = StatusResponse;

        fn http_request_method() -> http::Method {
            http::Method::GET
        }

        fn pageable() -> bool {
            false
        }

        fn path(&self) -> String {
            "/".to_string()
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
}
pub mod achievements {}
pub mod badges {}
pub mod characters {}
pub mod effects {}
pub mod events {}
pub mod grand_exchange {}
pub mod items {}
pub mod leaderboard {}
pub mod maps {
    use crate::api::Endpoint;

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
}
pub mod monsters {}
pub mod npcs {}
pub mod resources {}
pub mod tasks {}
pub mod token {}
