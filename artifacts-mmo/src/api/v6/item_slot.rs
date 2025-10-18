#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ItemSlot {
	#[default]
	Weapon,
	Shield,
	Helmet,
	BodyArmor,
	LegArmor,
	Boots,
	Ring1,
	Ring2,
	Amulet,
	Artifact1,
	Artifact2,
	Artifact3,
	Utility1,
	Utility2,
	Bag,
	Rune,
}
