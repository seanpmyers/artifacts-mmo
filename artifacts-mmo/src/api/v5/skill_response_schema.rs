#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SkillResponseSchema {
	/// SkillDataSchema
	data: super::skill_data_schema::SkillDataSchema,

}
