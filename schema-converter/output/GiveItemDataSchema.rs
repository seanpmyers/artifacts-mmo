#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveItemDataSchema {
	/// Character details of the sending character.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details.
	cooldown: #[serde(flatten)]
	CooldownSchema
,
	/// Items: Items given.
	items: Vec<SimpleItemSchema>,
	/// Character details of the receiving character.
	receiver_character: #[serde(flatten)]
	CharacterSchema
,

}
