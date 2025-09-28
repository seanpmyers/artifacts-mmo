#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GETransactionResponseSchema {
	/// GETransactionListSchema
	data: GETransactionListSchema,

}
