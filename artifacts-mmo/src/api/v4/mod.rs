pub mod accounts;
pub mod my_account {}
pub mod my_characters {
    use crate::api::{Endpoint, NoBody};

    use super::{
        characters::{Character, Location},
        maps::Map,
    };

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum CooldownReason {
        BuyBankExpansion,
        BuyGe,
        BuyNpc,
        CancelGe,
        ChristmasExchange,
        Crafting,
        DeleteItem,
        Deposit,
        DepositGold,
        Equip,
        Fight,
        Gathering,
        Movement,
        Recycling,
        #[default]
        Rest,
        SellGe,
        SellNpc,
        Task,
        Unequip,
        Use,
        Withdraw,
        WithdrawGold,
    }

    impl std::fmt::Display for CooldownReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    CooldownReason::BuyBankExpansion => "buy_bank_expansion",
                    CooldownReason::BuyGe => "buy_ge",
                    CooldownReason::BuyNpc => "buy_npc",
                    CooldownReason::CancelGe => "cancel_ge",
                    CooldownReason::ChristmasExchange => "christmas_exchange",
                    CooldownReason::Crafting => "crafting",
                    CooldownReason::DeleteItem => "delete_item",
                    CooldownReason::Deposit => "deposit",
                    CooldownReason::DepositGold => "deposit_gold",
                    CooldownReason::Equip => "equip",
                    CooldownReason::Fight => "fight",
                    CooldownReason::Gathering => "gathering",
                    CooldownReason::Movement => "movement",
                    CooldownReason::Recycling => "recycling",
                    CooldownReason::Rest => "rest",
                    CooldownReason::SellGe => "sell_ge",
                    CooldownReason::SellNpc => "sell_npc",
                    CooldownReason::Task => "task",
                    CooldownReason::Unequip => "unequip",
                    CooldownReason::Use => "use",
                    CooldownReason::Withdraw => "withdraw",
                    CooldownReason::WithdrawGold => "withdraw_gold",
                }
            )
        }
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct Cooldown {
        pub total_seconds: u32,
        pub remaining_seconds: u32,
        pub started_at: chrono::DateTime<chrono::Utc>,
        pub expiration: chrono::DateTime<chrono::Utc>,
        pub reason: CooldownReason,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct CharacterMovement {
        pub cooldown: Cooldown,
        pub destination: Map,
        pub character: Character,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct GetMyCharactersRequest {}

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct GetMyCharactersResponse {
        pub data: Vec<Character>,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct ActionMoveRequest {
        pub name: String,
        pub body: Location,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct ActionMoveResponse {
        pub data: CharacterMovement,
    }

    impl Endpoint for GetMyCharactersRequest {
        type Response = GetMyCharactersResponse;
        type RequestBody = NoBody;

        fn http_request_method() -> http::Method {
            http::Method::GET
        }

        fn path(&self) -> String {
            format!("/my/characters")
        }
    }

    impl Endpoint for ActionMoveRequest {
        type Response = ActionMoveResponse;
        type RequestBody = Location;

        fn http_request_method() -> http::Method {
            http::Method::POST
        }

        fn path(&self) -> String {
            format!("/my/{}/action/move", self.name)
        }

        fn request_body(&self) -> std::option::Option<Location> {
            Some(self.body.clone())
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
pub mod leaderboard;
pub mod maps;
pub mod monsters;
pub mod npcs {}
pub mod resources;
pub mod tasks {}
pub mod token {}
