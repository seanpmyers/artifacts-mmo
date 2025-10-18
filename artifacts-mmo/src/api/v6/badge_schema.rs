#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BadgeSchema {
	/// Code: Code of the badge. This is the badge's unique identifier (ID).
	code: String,
	/// Conditions: Conditions to get the badge.
	conditions: Vec<super::badge_condition_schema::BadgeConditionSchema>,
	/// Description: Description of the badge.
	description: String,
	/// Season: Season of the badge.
	season: Option<f64>,

}
