#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskSchema {
	/// Code: Task objective.
	code: String,
	/// Rewards for completing the task.
	#[serde(flatten)]
	rewards: super::rewards_schema::RewardsSchema,
	/// Total: The total required to complete the task.
	total: i32,
	/// Type: The type of task.
	#[serde(flatten)]
	#[serde(rename = "type")]
	r#type: super::task_type::TaskType,

}
