#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskResponseSchema {
	/// TaskDataSchema
	data: super::task_data_schema::TaskDataSchema,

}
