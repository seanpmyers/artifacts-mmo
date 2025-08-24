#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AccountDetails {
	/// Achievements Points: Achievement points.
	achievements_points: i32,
	/// Badges: Account badges.
	badges: TODO__Vec<>,
	/// Ban Reason: Ban reason.
	ban_reason: String,
	/// Banned: Banned.
	banned: bool,
	/// Member: Member status.
	member: bool,
	/// Skins: Skins owned.
	skins: TODO__Vec<>,
	/// Account status.
	status: TODO__NOT A SINGLE TYPE,
	/// Username: Username.
	username: String,

}
