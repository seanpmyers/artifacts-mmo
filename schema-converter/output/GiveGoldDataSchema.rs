#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveGoldDataSchema {
	/// Character details.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details.
	cooldown: #[serde(flatten)]
	CooldownSchema
,
	/// Quantity: Quantity of gold given.
	quantity: i32,
	/// Character details of the receiving character.
	receiver_character: #[serde(flatten)]
	CharacterSchema
,

}
