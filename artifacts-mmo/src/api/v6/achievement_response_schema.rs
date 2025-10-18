#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AchievementResponseSchema {
	/// AchievementSchema
	data: super::achievement_schema::AchievementSchema,

}
