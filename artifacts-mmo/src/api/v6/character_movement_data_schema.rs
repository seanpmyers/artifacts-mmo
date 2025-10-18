#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterMovementDataSchema {
	/// Character details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Destination details.
	#[serde(flatten)]
	destination: super::map_schema::MapSchema,
	/// Path: Path taken from start to destination (list of coordinates)
	path: Vec<serde_json::Value>,

}
