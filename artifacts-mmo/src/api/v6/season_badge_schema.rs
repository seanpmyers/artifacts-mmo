#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SeasonBadgeSchema {
	/// Code: Badge code.
	code: String,
	/// Description: Badge description.
	description: String,
	/// Required Points: Required achievement points to earn the badge.
	required_points: i32,

}
