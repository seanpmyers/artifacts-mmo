#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankGoldTransactionResponseSchema {
	/// BankGoldTransactionSchema
	data: super::bank_gold_transaction_schema::BankGoldTransactionSchema,

}
