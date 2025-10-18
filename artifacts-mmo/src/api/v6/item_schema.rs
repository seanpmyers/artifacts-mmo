#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ItemSchema {
	/// Code: Item code. This is the item's unique identifier (ID).
	code: String,
	/// Conditions: Item conditions. If applicable. Conditions for using or equipping the item.
	conditions: Vec<super::condition_schema::ConditionSchema>,
	/// Craft information. If applicable.
	craft: super::craft_schema::CraftSchema,
	/// Description: Item description.
	description: String,
	/// Effects: List of object effects. For equipment, it will include item stats.
	effects: Vec<super::simple_effect_schema::SimpleEffectSchema>,
	/// Level: Item level.
	level: i32,
	/// Name: Item name.
	name: String,
	/// Subtype: Item subtype.
	subtype: String,
	/// Tradeable: Item tradeable status. A non-tradeable item cannot be exchanged or sold.
	tradeable: bool,
	/// Type: Item type.
	#[serde(rename = "type")]
	r#type: String,

}
