use crate::api::{Endpoint, NoBody, PageInput, PageOutput};

use super::{accounts::MemberStatus, characters::Profile};

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CharacterSorting {
    Alchemy,
    #[default]
    Combat,
    Cooking,
    Fishing,
    GearCrafting,
    JewelryCrafting,
    Mining,
    WeaponCrafting,
    Woodcutting,
}

impl std::fmt::Display for CharacterSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CharacterSorting::Alchemy => "alchemy",
                CharacterSorting::Combat => "combat",
                CharacterSorting::Cooking => "cooking",
                CharacterSorting::Fishing => "fishing",
                CharacterSorting::GearCrafting => "gearcrafting",
                CharacterSorting::JewelryCrafting => "jewelrycrafting",
                CharacterSorting::Mining => "mining",
                CharacterSorting::WeaponCrafting => "weaponcrafting",
                CharacterSorting::Woodcutting => "woodcutting",
            }
        )
    }
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub enum AccountSorting {
    #[default]
    AchievementsPoints,
    Gold,
}

impl std::fmt::Display for AccountSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AccountSorting::AchievementsPoints => "achievements_points",
                AccountSorting::Gold => "gold",
            }
        )
    }
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterLeaderboard {
    pub alchemy_level: i32,
    pub alchemy_total_xp: i32,
    pub cooking_level: i32,
    pub cooking_total_xp: i32,
    pub fishing_level: i32,
    pub fisihing_total_xp: i32,
    pub gearcrafting_level: i32,
    pub gearcrafting_total_xp: i32,
    pub gold: i32,
    pub jewelrycrafting_level: i32,
    pub jewelrycrafting_total_xp: i32,
    pub level: i32,
    pub mining_level: i32,
    pub mining_total_xp: i32,
    pub name: String,
    pub position: u32,
    #[serde(flatten)]
    pub profile: Profile,
    pub status: MemberStatus,
    pub total_xp: i32,
    pub weaponcrafting_level: i32,
    pub weaponcrafting_total_xp: i32,
    pub woodcutting_level: i32,
    pub woodcutting_total_xp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AccountLeaderboard {
    pub account: String,
    pub achievements_points: u32,
    pub gold: u32,
    pub position: u32,
    pub status: MemberStatus,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetCharactersLeaderboardRequest {
    pub name: Option<String>,
    #[serde(flatten)]
    pub page_input: PageInput,
    pub sort: Option<CharacterSorting>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetCharactersLeaderboardResponse {
    pub data: Vec<CharacterLeaderboard>,
    #[serde(flatten)]
    pub page_output: PageOutput,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAccountsLeaderboardRequest {
    pub name: Option<String>,
    #[serde(flatten)]
    pub page_input: PageInput,
    pub sort: Option<AccountSorting>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAccountsLeaderboardResponse {
    pub data: Vec<AccountLeaderboard>,
    #[serde(flatten)]
    pub page_output: PageOutput,
}

impl Endpoint for GetCharactersLeaderboardRequest {
    type Response = GetCharactersLeaderboardResponse;
    type RequestBody = NoBody;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn path(&self) -> String {
        format!("/leaderboard/characters")
    }

    fn query_parameters(&self) -> Vec<(String, String)> {
        let mut result = self.page_input.to_tuples();

        if let Some(name) = &self.name {
            result.push((String::from("name"), name.to_string()));
        }
        if let Some(sort) = &self.sort {
            result.push((String::from("sort"), sort.to_string()));
        }

        result
    }
}

impl Endpoint for GetAccountsLeaderboardRequest {
    type Response = GetAccountsLeaderboardResponse;
    type RequestBody = NoBody;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn path(&self) -> String {
        format!("/leaderboard/accounts")
    }

    fn query_parameters(&self) -> Vec<(String, String)> {
        let mut result = self.page_input.to_tuples();

        if let Some(name) = &self.name {
            result.push((String::from("name"), name.clone()));
        }
        if let Some(sort) = &self.sort {
            result.push((String::from("sort"), sort.to_string()));
        }

        result
    }
}
