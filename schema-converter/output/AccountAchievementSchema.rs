#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AccountAchievementSchema {
	/// Code: Code of the achievement. 
	code: String,
	/// Completed At: Completed at.
	completed_at: TODO__NOT A SINGLE TYPE,
	/// Current: Current progress.
	current: i32,
	/// Description: Description of the achievement.
	description: String,
	/// Name: Name of the achievement.
	name: String,
	/// Points: Points of the achievement. Used for the leaderboard.
	points: i32,
	/// Rewards.
	rewards: TODO__NOT A SINGLE TYPE,
	/// Target: Target of the achievement.
	target: TODO__NOT A SINGLE TYPE,
	/// Total: Total to do.
	total: i32,
	/// Type of achievement.
	AccountAchievementSchema_type: TODO__NOT A SINGLE TYPE,

}
