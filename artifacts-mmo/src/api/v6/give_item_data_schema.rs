#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveItemDataSchema {
	/// Character details of the sending character.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Items: Items given.
	items: Vec<super::simple_item_schema::SimpleItemSchema>,
	/// Character details of the receiving character.
	#[serde(flatten)]
	receiver_character: super::character_schema::CharacterSchema,

}
