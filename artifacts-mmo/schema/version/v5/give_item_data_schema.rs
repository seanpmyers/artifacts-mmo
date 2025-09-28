#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveItemDataSchema {
	/// Character details of the sending character.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: CooldownSchema,
	/// Items: Items given.
	items: Vec<SimpleItemSchema>,
	/// Character details of the receiving character.
	#[serde(flatten)]
	receiver_character: CharacterSchema,

}
