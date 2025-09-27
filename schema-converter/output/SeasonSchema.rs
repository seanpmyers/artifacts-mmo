#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SeasonSchema {
	/// Badges: Season badges with required achievement points.
	badges: Vec<SeasonBadgeSchema>,
	/// Name: Season name.
	name: String,
	/// Number: Season number.
	number: i32,
	/// Skins: Season skins with required achievement points.
	skins: Vec<SeasonSkinSchema>,
	/// Start Date: Season start date.
	start_date: chrono::DateTime<chrono::Utc>,

}
