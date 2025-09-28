#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterFightResponseSchema {
	/// CharacterFightDataSchema
	data: CharacterFightDataSchema,

}
