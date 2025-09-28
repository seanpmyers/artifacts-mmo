#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GECreateOrderTransactionResponseSchema {
	/// GEOrderTransactionSchema
	data: super::georder_transaction_schema::GEOrderTransactionSchema,

}
