#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DropRateSchema {
	/// Item code: Item code.
	code: String,
	/// Max Quantity: Maximum quantity.
	max_quantity: i32,
	/// Min Quantity: Minimum quantity.
	min_quantity: i32,
	/// Rate: Chance rate. (1/rate)
	rate: i32,

}
