#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankItemTransactionSchema {
	/// Bank: Items in your banks.
	bank: Vec<SimpleItemSchema>,
	/// Player details.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: CooldownSchema,
	/// Items: Items details.
	items: Vec<SimpleItemSchema>,

}
