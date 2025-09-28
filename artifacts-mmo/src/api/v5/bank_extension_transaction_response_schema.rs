#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankExtensionTransactionResponseSchema {
	/// BankExtensionTransactionSchema
	data: super::bank_extension_transaction_schema::BankExtensionTransactionSchema,

}
