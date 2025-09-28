#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GEOrderCreationrSchema {
	/// Code: Item code.
	code: String,
	/// Price: Item price per unit.
	price: i32,
	/// Quantity: Item quantity.
	quantity: i32,

}
