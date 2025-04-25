use crate::api::Endpoint;

use super::{
    achievements::{AchievementRewards, AchievementType},
    characters::Character,
};

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAccountRequest {
    pub account: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAccountResponse {
    pub data: AccountDetails,
}

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

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAccountAchievementsRequest {
    pub account: String,
    #[serde(rename = "type")]
    pub achievement_type: Option<AchievementType>,
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
    pub achivement_type: AchievementType,
    pub code: String,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub name: String,
    pub points: i32,
    pub rewards: AchievementRewards,
    pub target: String,
    pub total: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAccountCharactersRequest {
    pub account: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAccountCharactersResponse {
    pub data: Vec<Character>,
}

impl Endpoint for CreateAccountRequest {
    type Response = CreateAccountResponse;

    fn http_request_method() -> http::Method {
        http::Method::POST
    }

    fn path(&self) -> String {
        "/accounts/create".to_string()
    }
}

impl Endpoint for GetAccountAchievementsRequest {
    type Response = GetAccountAchievementsResponse;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}/achievements", self.account)
    }

    fn query(&self) -> Option<String> {
        let mut query: String = format!("?size={}", self.size.unwrap_or(Self::page_size()));
        if let Some(achievement_type) = &self.achievement_type {
            query.push_str(&format!("&type={}", achievement_type));
        }
        if let Some(completed) = self.completed {
            query.push_str(&format!("&completed={}", completed));
        }
        Some(query)
    }
}

impl Endpoint for GetAccountRequest {
    type Response = GetAccountResponse;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn path(&self) -> String {
        format!("/accounts/{}", self.account)
    }
}

impl Endpoint for GetAccountCharactersRequest {
    type Response = GetAccountCharactersResponse;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}/characters", self.account)
    }
}
