#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RecyclingSchema {
	/// Item code: Item code.
	code: String,
	/// Quantity: Quantity of items to recycle.
	quantity: i32,

}
