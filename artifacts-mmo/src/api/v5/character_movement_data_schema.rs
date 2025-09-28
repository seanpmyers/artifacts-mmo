#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterMovementDataSchema {
	/// Character details.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details
	#[serde(flatten)]
	cooldown: CooldownSchema,
	/// Destination details.
	#[serde(flatten)]
	destination: MapSchema,

}
