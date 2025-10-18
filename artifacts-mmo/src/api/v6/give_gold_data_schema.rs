#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveGoldDataSchema {
	/// Character details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Quantity: Quantity of gold given.
	quantity: i32,
	/// Character details of the receiving character.
	#[serde(flatten)]
	receiver_character: super::character_schema::CharacterSchema,

}
