#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AccountAchievementSchema {
	/// Code: Code of the achievement. 
	code: String,
	/// Completed At: Completed at.
	completed_at: chrono::DateTime<chrono::Utc>,
	/// Current: Current progress.
	current: i32,
	/// Description: Description of the achievement.
	description: String,
	/// Name: Name of the achievement.
	name: String,
	/// Points: Points of the achievement. Used for the leaderboard.
	points: i32,
	/// Rewards.
	#[serde(flatten)]
	rewards: super::achievement_rewards_schema::AchievementRewardsSchema,
	/// Target: Target of the achievement.
	target: String,
	/// Total: Total to do.
	total: i32,
	/// Type of achievement.
	#[serde(flatten)]
	#[serde(rename = "type")]
	r#type: super::achievement_type::AchievementType,

}
