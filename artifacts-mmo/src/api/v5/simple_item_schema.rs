#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SimpleItemSchema {
	/// Code: Item code.
	code: String,
	/// Quantity: Item quantity.
	quantity: i32,

}
