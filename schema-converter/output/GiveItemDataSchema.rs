#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveItemDataSchema {
	/// Character details of the sending character.
	character: TODO__NOT A SINGLE TYPE,
	/// Cooldown details.
	cooldown: TODO__NOT A SINGLE TYPE,
	/// Items: Items given.
	items: TODO__Vec<>,
	/// Character details of the receiving character.
	receiver_character: TODO__NOT A SINGLE TYPE,

}
