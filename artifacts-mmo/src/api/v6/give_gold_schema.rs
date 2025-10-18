#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveGoldSchema {
	/// Character: Character name. The name of the character who will receive the gold.
	character: String,
	/// Quantity: Gold quantity.
	quantity: i32,

}
