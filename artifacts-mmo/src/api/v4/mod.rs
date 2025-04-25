pub mod accounts;
pub mod my_account {}
pub mod my_characters {
    use crate::api::Endpoint;

    use super::characters::Character;

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct GetMyCharactersRequest {}

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct GetMyCharactersResponse {
        pub data: Vec<Character>,
    }

    impl Endpoint for GetMyCharactersRequest {
        type Response = GetMyCharactersResponse;

        fn http_request_method() -> http::Method {
            http::Method::GET
        }

        fn path(&self) -> String {
            format!("/my/characters")
        }
    }
}
pub mod status;
pub mod achievements {
    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum AchievementType {
        CombatDrop,
        CombatKill,
        CombatLevel,
        Crafting,
        Gathering,
        #[default]
        Other,
        Recycling,
        Task,
        Use,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct AchievementRewards {
        pub gold: i32,
    }

    impl std::fmt::Display for AchievementType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    AchievementType::CombatDrop => "combat_drop",
                    AchievementType::CombatKill => "combat_kill",
                    AchievementType::CombatLevel => "combat_level",
                    AchievementType::Crafting => "crafting",
                    AchievementType::Gathering => "gathering",
                    AchievementType::Other => "other",
                    AchievementType::Recycling => "recycling",
                    AchievementType::Task => "task",
                    AchievementType::Use => "use",
                }
            )
        }
    }
}
pub mod badges {}
pub mod characters;
pub mod effects {}
pub mod events {}
pub mod grand_exchange {}
pub mod items {}
pub mod leaderboard {
    use crate::api::{Endpoint, PageInput, PageOutput};

    use super::{accounts::MemberStatus, characters::Profile};

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub enum SortingOption {
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

    impl std::fmt::Display for SortingOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    SortingOption::Alchemy => "alchemy",
                    SortingOption::Combat => "combat",
                    SortingOption::Cooking => "cooking",
                    SortingOption::Fishing => "fishing",
                    SortingOption::GearCrafting => "gearcrafting",
                    SortingOption::JewelryCrafting => "jewelrycrafting",
                    SortingOption::Mining => "mining",
                    SortingOption::WeaponCrafting => "weaponcrafting",
                    SortingOption::Woodcutting => "woodcutting",
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
        pub sort: Option<SortingOption>,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct GetCharactersLeaderboardResponse {
        pub data: Vec<CharacterLeaderboard>,
        #[serde(flatten)]
        pub page_output: PageOutput,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct GetAccountsLeaderboardResponse {
        pub data: Vec<AccountLeaderboard>,
        #[serde(flatten)]
        pub page_output: PageOutput,
    }

    impl Endpoint for GetCharactersLeaderboardRequest {
        type Response = GetCharactersLeaderboardResponse;

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
}
pub mod maps;
pub mod monsters;
pub mod npcs {}
pub mod resources;
pub mod tasks {}
pub mod token {}
