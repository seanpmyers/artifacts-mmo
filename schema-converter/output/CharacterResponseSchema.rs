#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterResponseSchema {
	/// CharacterSchema
	data: CharacterSchema,

}
