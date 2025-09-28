#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GEOrderCreatedSchema {
	/// Code: Item code.
	code: String,
	/// Created At: Order created at.
	created_at: chrono::DateTime<chrono::Utc>,
	/// Id: Order id.
	id: String,
	/// Price: Item price per unit.
	price: i32,
	/// Quantity: Item quantity.
	quantity: i32,
	/// Tax: Listing tax (3%, minimum 1)
	tax: i32,
	/// Total Price: Total price.
	total_price: i32,

}
