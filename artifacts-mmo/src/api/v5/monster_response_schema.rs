#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MonsterResponseSchema {
	/// MonsterSchema
	data: super::monster_schema::MonsterSchema,

}
