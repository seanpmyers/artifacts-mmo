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
	rewards: TODO__NOT A SINGLE TYPE,
	/// Skill: Skill required to complete the task.
	skill: TODO__NOT A SINGLE TYPE,
	/// Type: The type of task.
	TaskFullSchema_type: TODO__NOT A SINGLE TYPE,

}
