#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankItemTransactionSchema {
	/// Bank: Items in your banks.
	bank: Vec<super::simple_item_schema::SimpleItemSchema>,
	/// Player details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Items: Items details.
	items: Vec<super::simple_item_schema::SimpleItemSchema>,

}
