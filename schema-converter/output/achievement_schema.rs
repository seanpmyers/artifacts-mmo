#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AchievementSchema {
	/// Code: Code of the achievement. 
	code: String,
	/// Description: Description of the achievement.
	description: String,
	/// Name: Name of the achievement.
	name: String,
	/// Points: Points of the achievement. Used for the leaderboard.
	points: i32,
	/// Rewards.
	#[serde(flatten)]
	rewards: AchievementRewardsSchema,
	/// Target: Target of the achievement.
	target: Option<String>,
	/// Total: Total to do.
	total: i32,
	/// Type of achievement.
	#[serde(flatten)]
	AchievementSchema_type: AchievementType,

}
