use crate::api::{Endpoint, EndpointResponse};
use log::error;

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

    fn call(
        &mut self,
        bearer_token: String,
        http_client: &mut ureq::Agent,
    ) -> EndpointResponse<StatusResponse> {
        let response: Result<http::Response<ureq::Body>, ureq::Error> =
            Self::make_api_call::<StatusRequest>(self, http_client, vec![], bearer_token);
        match response {
            Ok(mut response) => {
                let body = response.body_mut().read_json::<StatusResponse>();
                match body {
                    Ok(status) => return EndpointResponse::Success(status),
                    Err(error) => error!("{}", error),
                }
            }
            Err(error) => error!("{}", error),
        }
        EndpointResponse::Error
    }

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
