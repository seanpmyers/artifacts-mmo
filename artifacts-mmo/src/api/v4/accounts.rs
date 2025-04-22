use crate::api::{Endpoint, EndpointResponse};

use super::achievements::{AchievementRewards, AchivementType};

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAccountRequest {}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAccountResponse {}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct AccountDetails {
    pub achievement_points: u32,
    pub badges: Vec<Badge>,
    pub ban_reason: String,
    pub banned: bool,
    pub status: MemberStatus,
    pub subscribed: bool,
    pub username: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemberStatus {
    Founder,
    GoldFounder,
    #[default]
    Standard,
    VipFound,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Badge {}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct CreateAccountRequest {
    pub email: Option<String>,
    pub password: String,
    pub username: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct CreateAccountResponse {}

impl Endpoint for CreateAccountRequest {
    fn call(
        &mut self,
        bearer_token: String,
        http_client: &mut ureq::Agent,
    ) -> EndpointResponse<CreateAccountResponse> {
        todo!()
    }

    fn http_request_method() -> http::Method {
        http::Method::POST
    }

    fn pageable() -> bool {
        false
    }

    fn path(&self) -> String {
        "/accounts/create".to_string()
    }

    type Response = CreateAccountResponse;

    fn query(&self) -> Option<String> {
        None
    }

    fn request_body(&self) -> Option<CreateAccountRequest> {
        Some(self.clone())
    }
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAccountAchievementsRequest {
    pub account: String,
    #[serde(rename = "type")]
    pub achivement_type: Option<AchivementType>,
    pub completed: Option<bool>,
    pub page: Option<u32>,
    pub size: Option<u32>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAccountAchievementsResponse {
    pub data: Vec<AccountAchievement>,
    pub page: i32,
    pub pages: i32,
    pub size: i32,
    pub total: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct AccountAchievement {
    #[serde(rename = "type")]
    pub achivement_type: AchivementType,
    pub code: String,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub name: String,
    pub points: i32,
    pub rewards: AchievementRewards,
    pub target: String,
    pub total: i32,
}
