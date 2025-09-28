#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MyAccountDetails {
	/// Achievements Points: Achievement points.
	achievements_points: i32,
	/// Badges: Account badges.
	badges: Vec<serde_json::Value>,
	/// Ban Reason: Ban reason.
	ban_reason: String,
	/// Banned: Banned.
	banned: bool,
	/// Email: Email.
	email: String,
	/// Gems: Gems.
	gems: i32,
	/// Member: Member status.
	member: bool,
	/// Member Expiration: Member expiration date.
	member_expiration: Option<String>,
	/// Skins: Skins owned.
	skins: Vec<serde_json::Value>,
	/// Account status.
	#[serde(flatten)]
	status: super::account_status::AccountStatus,
	/// Username: Username.
	username: String,

}
