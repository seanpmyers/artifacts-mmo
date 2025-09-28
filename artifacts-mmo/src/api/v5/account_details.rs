#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AccountDetails {
	/// Achievements Points: Achievement points.
	achievements_points: i32,
	/// Badges: Account badges.
	badges: Vec<serde_json::Value>,
	/// Ban Reason: Ban reason.
	ban_reason: String,
	/// Banned: Banned.
	banned: bool,
	/// Member: Member status.
	member: bool,
	/// Skins: Skins owned.
	skins: Vec<serde_json::Value>,
	/// Account status.
	#[serde(flatten)]
	status: super::account_status::AccountStatus,
	/// Username: Username.
	username: String,

}
