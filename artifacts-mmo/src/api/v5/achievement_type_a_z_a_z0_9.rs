#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AchievementTypeAZAZ0_9 {
    #[default]
    CombatKill,
    CombatDrop,
    CombatLevel,
    Gathering,
    Crafting,
    Recycling,
    Task,
    Other,
    Use,
}
