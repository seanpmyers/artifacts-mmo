pub mod handler;

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{ApiEndpoint, HttpRequestMethod},
};

pub const STATUS: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/",
    http_request_method: HttpRequestMethod::GET,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub data: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub announcements: Option<Vec<Announcements>>,
    pub characters_online: i32,
    pub last_wipe: String,
    pub next_wipe: String,
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Announcements {
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ServerStatus {
    Online,
    Unknown,
    Offline,
}

impl fmt::Display for ServerStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerStatus::Online => write!(formatter, "Online"),
            ServerStatus::Unknown => write!(formatter, "Unknown"),
            ServerStatus::Offline => write!(formatter, "Offline"),
        }
    }
}
