#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankGoldTransactionResponseSchema {
	/// BankGoldTransactionSchema
	data: BankGoldTransactionSchema,

}
