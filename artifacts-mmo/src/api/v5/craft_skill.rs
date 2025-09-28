#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CraftSkill {
	#[default]
	Weaponcrafting,
	Gearcrafting,
	Jewelrycrafting,
	Cooking,
	Woodcutting,
	Mining,
	Alchemy,
}
