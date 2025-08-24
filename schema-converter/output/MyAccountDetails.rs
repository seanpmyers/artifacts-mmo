#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MyAccountDetails {
	/// Achievements Points: Achievement points.
	achievements_points: i32,
	/// Badges: Account badges.
	badges: TODO__Vec<>,
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
	member_expiration: TODO__NOT A SINGLE TYPE,
	/// Skins: Skins owned.
	skins: TODO__Vec<>,
	/// Account status.
	status: TODO__NOT A SINGLE TYPE,
	/// Username: Username.
	username: String,

}
