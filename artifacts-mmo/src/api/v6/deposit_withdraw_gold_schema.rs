#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DepositWithdrawGoldSchema {
	/// Quantity: Quantity of gold.
	quantity: i32,

}
