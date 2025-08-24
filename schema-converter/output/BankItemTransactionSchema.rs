#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankItemTransactionSchema {
	/// Bank: Items in your banks.
	bank: TODO__Vec<>,
	/// Player details.
	character: TODO__NOT A SINGLE TYPE,
	/// Cooldown details.
	cooldown: TODO__NOT A SINGLE TYPE,
	/// Items: Items details.
	items: TODO__Vec<>,

}
