#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterMovementResponseSchema {
	/// CharacterMovementDataSchema
	data: CharacterMovementDataSchema,

}
