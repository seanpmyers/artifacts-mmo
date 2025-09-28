#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterRestResponseSchema {
	/// CharacterRestDataSchema
	data: CharacterRestDataSchema,

}
