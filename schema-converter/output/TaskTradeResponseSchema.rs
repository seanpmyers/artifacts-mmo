#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskTradeResponseSchema {
	/// TaskTradeDataSchema
	data: TaskTradeDataSchema,

}
