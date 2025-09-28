#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NpcMerchantBuySchema {
	/// Code: Item code.
	code: String,
	/// Quantity: Item quantity.
	quantity: i32,

}
