#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MyAccountDetails {
	/// Achievements Points: Achievement points.
	achievements_points: i32,
	/// Badges: Account badges.
	badges: Vec<serde::Value>,
	/// Ban Reason: Ban reason.
	ban_reason: String,
	/// Banned: Banned.
	banned: bool,
	/// Email: Email.
	email: String__TODO__email,
	/// Gems: Gems.
	gems: i32,
	/// Member: Member status.
	member: bool,
	/// Member Expiration: Member expiration date.
	member_expiration: ,
	/// Skins: Skins owned.
	skins: Vec<serde::Value>,
	/// Account status.
	status: #[serde(flatten)]
	AccountStatus
,
	/// Username: Username.
	username: String,

}
