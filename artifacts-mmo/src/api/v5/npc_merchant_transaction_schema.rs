#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NpcMerchantTransactionSchema {
	/// Character details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Transaction details.
	#[serde(flatten)]
	transaction: super::npc_item_transaction_schema::NpcItemTransactionSchema,

}
