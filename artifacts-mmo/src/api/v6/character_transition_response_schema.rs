#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterTransitionResponseSchema {
	/// CharacterTransitionDataSchema
	data: super::character_transition_data_schema::CharacterTransitionDataSchema,

}
