#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterMovementResponseSchema {
	/// CharacterMovementDataSchema
	data: super::character_movement_data_schema::CharacterMovementDataSchema,

}
