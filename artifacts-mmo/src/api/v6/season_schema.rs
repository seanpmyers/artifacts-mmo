#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SeasonSchema {
	/// Badges: Season badges with required achievement points.
	badges: Vec<super::season_badge_schema::SeasonBadgeSchema>,
	/// Name: Season name.
	name: String,
	/// Number: Season number.
	number: i32,
	/// Skins: Season skins with required achievement points.
	skins: Vec<super::season_skin_schema::SeasonSkinSchema>,
	/// Start Date: Season start date.
	start_date: chrono::DateTime<chrono::Utc>,

}
