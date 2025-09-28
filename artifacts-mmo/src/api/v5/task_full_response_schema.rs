#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskFullResponseSchema {
	/// TaskFullSchema
	data: super::task_full_schema::TaskFullSchema,

}
