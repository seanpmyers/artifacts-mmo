#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CraftingSchema {
	/// Craft code: Craft code.
	code: String,
	/// Quantity: Quantity of items to craft.
	quantity: i32,

}
