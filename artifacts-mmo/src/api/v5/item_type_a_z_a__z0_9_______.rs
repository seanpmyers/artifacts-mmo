#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ItemType_a_zA_Z0_9_______ {
	#[default]
	Utility,
	BodyArmor,
	Weapon,
	Resource,
	LegArmor,
	Helmet,
	Boots,
	Shield,
	Amulet,
	Ring,
	Artifact,
	Currency,
	Consumable,
	Rune,
	Bag,
}
