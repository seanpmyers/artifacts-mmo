#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NpcMerchantTransactionResponseSchema {
	/// NpcMerchantTransactionSchema
	data: super::npc_merchant_transaction_schema::NpcMerchantTransactionSchema,

}
