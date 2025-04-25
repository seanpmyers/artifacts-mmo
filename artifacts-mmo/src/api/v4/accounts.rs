use crate::api::{Endpoint, PageInput, PageOutput};

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
    #[serde(flatten)]
    pub page_input: PageInput,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAccountAchievementsResponse {
    pub data: Vec<AccountAchievement>,
    #[serde(flatten)]
    pub page_output: PageOutput,
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

    fn query_parameters(&self) -> Vec<(String, String)> {
        let mut result = self.page_input.to_tuples();

        if let Some(x) = &self.achievement_type {
            result.push((String::from("type"), x.to_string()));
        }
        if let Some(completed) = &self.completed {
            result.push((String::from("completed"), completed.to_string()));
        }

        result
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
