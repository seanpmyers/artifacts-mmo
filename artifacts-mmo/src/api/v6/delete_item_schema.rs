#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DeleteItemSchema {
	/// Player details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Item details.
	#[serde(flatten)]
	item: super::simple_item_schema::SimpleItemSchema,

}
