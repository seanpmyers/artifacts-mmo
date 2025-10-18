#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CharacterLeaderboardType {
	#[default]
	Combat,
	Woodcutting,
	Mining,
	Fishing,
	Weaponcrafting,
	Gearcrafting,
	Jewelrycrafting,
	Cooking,
	Alchemy,
}
