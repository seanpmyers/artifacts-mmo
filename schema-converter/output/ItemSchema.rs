#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ItemSchema {
	/// Code: Item code. This is the item's unique identifier (ID).
	code: String,
	/// Conditions: Item conditions. If applicable. Conditions for using or equipping the item.
	conditions: TODO__Vec<>,
	/// Craft information. If applicable.
	craft: TODO__NOT A SINGLE TYPE,
	/// Description: Item description.
	description: String,
	/// Effects: List of object effects. For equipment, it will include item stats.
	effects: TODO__Vec<>,
	/// Level: Item level.
	level: i32,
	/// Name: Item name.
	name: String,
	/// Subtype: Item subtype.
	subtype: String,
	/// Tradeable: Item tradeable status. A non-tradeable item cannot be exchanged or sold.
	tradeable: bool,
	/// Type: Item type.
	ItemSchema_type: String,

}
