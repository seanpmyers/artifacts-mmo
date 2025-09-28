#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AccountLeaderboardSchema {
	/// Account: Account name.
	account: String,
	/// Achievements Points: Achievements points.
	achievements_points: i32,
	/// Gold: Gold in the account.
	gold: i32,
	/// Position: Position in the leaderboard.
	position: i32,
	/// Member status.
	#[serde(flatten)]
	status: super::account_status::AccountStatus,

}
