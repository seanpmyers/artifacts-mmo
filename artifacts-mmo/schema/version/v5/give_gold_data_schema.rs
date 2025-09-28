#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveGoldDataSchema {
	/// Character details.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: CooldownSchema,
	/// Quantity: Quantity of gold given.
	quantity: i32,
	/// Character details of the receiving character.
	#[serde(flatten)]
	receiver_character: CharacterSchema,

}
