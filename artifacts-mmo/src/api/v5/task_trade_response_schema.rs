#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskTradeResponseSchema {
	/// TaskTradeDataSchema
	data: super::task_trade_data_schema::TaskTradeDataSchema,

}
