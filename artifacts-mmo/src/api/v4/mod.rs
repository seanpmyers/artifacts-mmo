pub mod accounts;
pub mod my_account {}
pub mod my_characters {}
pub mod status;
pub mod achievements {
    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum AchivementType {
        CombatDrop,
        CombatKill,
        CombatLevel,
        Crafting,
        Gathering,
        #[default]
        Other,
        Recyling,
        Task,
        Use,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct AchievementRewards {
        pub gold: i32,
    }
}
pub mod badges {}
pub mod characters;
pub mod effects {}
pub mod events {}
pub mod grand_exchange {}
pub mod items {}
pub mod leaderboard {}
pub mod maps;
pub mod monsters {}
pub mod npcs {}
pub mod resources;
pub mod tasks {}
pub mod token {}
