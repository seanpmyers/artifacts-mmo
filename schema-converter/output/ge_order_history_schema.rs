#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GeOrderHistorySchema {
	/// Buyer: Buyer account name.
	buyer: String,
	/// Code: Item code.
	code: String,
	/// Order Id: Order id.
	order_id: String,
	/// Price: Item price per unit.
	price: i32,
	/// Quantity: Item quantity.
	quantity: i32,
	/// Seller: Seller account name.
	seller: String,
	/// Sold At: Sale datetime.
	sold_at: chrono::DateTime<chrono::Utc>,

}
