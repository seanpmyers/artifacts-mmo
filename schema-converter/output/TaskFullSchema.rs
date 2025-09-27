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
	rewards: #[serde(flatten)]
	RewardsSchema
,
	/// Skill: Skill required to complete the task.
	skill: ,
	/// Type: The type of task.
	TaskFullSchema_type: #[serde(flatten)]
	TaskType
,

}
