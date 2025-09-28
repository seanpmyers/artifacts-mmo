#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DropSchema {
	/// Code: The code of the item.
	code: String,
	/// Quantity: The quantity of the item.
	quantity: i32,

}
