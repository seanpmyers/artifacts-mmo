#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NpcItemTransactionSchema {
	/// Code: Item code.
	code: String,
	/// Currency: Currency used for the transaction.
	currency: String,
	/// Price: Item price.
	price: i32,
	/// Quantity: Item quantity.
	quantity: i32,
	/// Total Price: Total price of the transaction.
	total_price: i32,

}
