#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterRestResponseSchema {
	/// CharacterRestDataSchema
	data: super::character_rest_data_schema::CharacterRestDataSchema,

}
