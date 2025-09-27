#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterMovementDataSchema {
	/// Character details.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details
	cooldown: #[serde(flatten)]
	CooldownSchema
,
	/// Destination details.
	destination: #[serde(flatten)]
	MapSchema
,

}
