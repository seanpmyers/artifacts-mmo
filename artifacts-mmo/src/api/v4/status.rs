use crate::api::{Endpoint, NoBody};

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct StatusRequest {}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct StatusResponse {
    pub data: GameStatus,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GameStatus {
    pub announcements: Option<Vec<Announcements>>,
    pub characters_online: i32,
    pub last_wipe: String,
    pub max_level: i32,
    pub next_wipe: String,
    pub server_time: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub version: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Announcements {
    pub message: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
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
    type RequestBody = NoBody;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn path(&self) -> String {
        "/".to_string()
    }
}
