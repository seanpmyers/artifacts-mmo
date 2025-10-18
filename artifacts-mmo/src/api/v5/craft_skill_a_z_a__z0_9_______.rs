#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CraftSkillAZAZ0_9 {
    #[default]
    Weaponcrafting,
    Gearcrafting,
    Jewelrycrafting,
    Cooking,
    Woodcutting,
    Mining,
    Alchemy,
}
