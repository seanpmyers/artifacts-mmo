#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankItemTransactionResponseSchema {
	/// BankItemTransactionSchema
	data: super::bank_item_transaction_schema::BankItemTransactionSchema,

}
