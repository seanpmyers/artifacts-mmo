#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskFullSchema {
	/// Code: Task objective.
	code: String,
	/// Level: Task level.
	level: i32,
	/// Max Quantity: Maximum amount of task.
	max_quantity: i32,
	/// Min Quantity: Minimum amount of task.
	min_quantity: i32,
	/// Rewards.
	#[serde(flatten)]
	rewards: super::rewards_schema::RewardsSchema,
	/// Skill: Skill required to complete the task.
	skill: Option<String>,
	/// Type: The type of task.
	#[serde(flatten)]
	#[serde(rename = "type")]
	r#type: super::task_type::TaskType,

}
