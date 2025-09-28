#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GEOrderSchema {
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
	/// Seller: Seller account name.
	seller: String,

}
