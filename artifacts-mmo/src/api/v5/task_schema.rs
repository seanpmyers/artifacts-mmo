#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskSchema {
	/// Code: Task objective.
	code: String,
	/// Rewards for completing the task.
	#[serde(flatten)]
	rewards: RewardsSchema,
	/// Total: The total required to complete the task.
	total: i32,
	/// Type: The type of task.
	#[serde(flatten)]
	TaskSchema_type: TaskType,

}
