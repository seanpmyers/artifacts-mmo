#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskCancelledResponseSchema {
	/// TaskCancelledSchema
	data: super::task_cancelled_schema::TaskCancelledSchema,

}
