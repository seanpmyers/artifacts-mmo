#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccountLeaderboardType {
	#[default]
	AchievementsPoints,
	Gold,
}
