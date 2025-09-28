#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GEBuyOrderSchema {
	/// Id: Order id.
	id: String,
	/// Quantity: Item quantity.
	quantity: i32,

}
