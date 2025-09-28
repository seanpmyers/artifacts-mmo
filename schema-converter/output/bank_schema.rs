#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankSchema {
	/// Expansions: Bank expansions.
	expansions: i32,
	/// Gold: Quantity of gold in your bank.
	gold: i32,
	/// Next Expansion Cost: Next expansion cost.
	next_expansion_cost: i32,
	/// Slots: Maximum slots in your bank.
	slots: i32,

}
