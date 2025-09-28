#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskResponseSchema {
	/// TaskDataSchema
	data: TaskDataSchema,

}
