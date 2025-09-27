#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankItemTransactionSchema {
	/// Bank: Items in your banks.
	bank: Vec<SimpleItemSchema>,
	/// Player details.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details.
	cooldown: #[serde(flatten)]
	CooldownSchema
,
	/// Items: Items details.
	items: Vec<SimpleItemSchema>,

}
