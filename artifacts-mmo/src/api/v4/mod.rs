pub mod accounts;
pub mod my_account {}
pub mod my_characters;
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
