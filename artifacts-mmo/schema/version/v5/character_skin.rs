#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CharacterSkin {
	#[default]
	Men1,
	Men2,
	Men3,
	Women1,
	Women2,
	Women3,
	Corrupted1,
	Zombie1,
}
