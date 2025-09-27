#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankExtensionTransactionResponseSchema {
	/// BankExtensionTransactionSchema
	data: BankExtensionTransactionSchema,

}
