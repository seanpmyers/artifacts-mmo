#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterResponseSchema {
	/// CharacterSchema
	data: super::character_schema::CharacterSchema,

}
