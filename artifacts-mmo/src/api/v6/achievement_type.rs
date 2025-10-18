#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AchievementType {
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
