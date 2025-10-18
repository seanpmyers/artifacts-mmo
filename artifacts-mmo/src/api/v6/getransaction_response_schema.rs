#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GETransactionResponseSchema {
	/// GETransactionListSchema
	data: super::getransaction_list_schema::GETransactionListSchema,

}
