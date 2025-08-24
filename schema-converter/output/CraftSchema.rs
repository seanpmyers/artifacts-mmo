#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CraftSchema {
	/// Items: List of items required to craft the item.
	items: TODO__Vec<>,
	/// Level: The skill level required to craft the item.
	level: i32,
	/// Quantity: Quantity of items crafted.
	quantity: i32,
	/// Skill code: Skill required to craft the item.
	skill: TODO__NOT A SINGLE TYPE,

}
