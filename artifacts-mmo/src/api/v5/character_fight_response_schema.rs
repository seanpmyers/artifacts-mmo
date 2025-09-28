#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterFightResponseSchema {
	/// CharacterFightDataSchema
	data: super::character_fight_data_schema::CharacterFightDataSchema,

}
