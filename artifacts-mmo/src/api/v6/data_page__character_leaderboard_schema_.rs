#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DataPage_CharacterLeaderboardSchema_ {
	/// Data
	data: Vec<super::character_leaderboard_schema::CharacterLeaderboardSchema>,
	/// Page
	page: i32,
	/// Pages
	pages: i32,
	/// Size
	size: i32,
	/// Total
	total: i32,

}
