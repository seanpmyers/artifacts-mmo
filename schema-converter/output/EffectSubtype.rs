#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EffectSubtype {
	#[default]
	Stat,
	Other,
	Heal,
	Buff,
	Debuff,
	Special,
	Gathering,
	Teleport,
	Gold,
}
