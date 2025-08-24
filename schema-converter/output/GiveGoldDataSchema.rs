#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveGoldDataSchema {
	/// Character details.
	character: TODO__NOT A SINGLE TYPE,
	/// Cooldown details.
	cooldown: TODO__NOT A SINGLE TYPE,
	/// Quantity: Quantity of gold given.
	quantity: i32,
	/// Character details of the receiving character.
	receiver_character: TODO__NOT A SINGLE TYPE,

}
