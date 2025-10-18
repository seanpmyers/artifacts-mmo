#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeSkinResponseSchema {
	/// ChangeSkinCharacterDataSchema
	data: super::change_skin_character_data_schema::ChangeSkinCharacterDataSchema,

}
