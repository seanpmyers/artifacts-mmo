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
	rewards: #[serde(flatten)]
	AchievementRewardsSchema
,
	/// Target: Target of the achievement.
	target: ,
	/// Total: Total to do.
	total: i32,
	/// Type of achievement.
	AchievementSchema_type: #[serde(flatten)]
	AchievementType
,

}
