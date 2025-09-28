#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AchievementResponseSchema {
	/// AchievementSchema
	data: AchievementSchema,

}
