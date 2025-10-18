#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterTransitionDataSchema {
	/// Character details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Destination map details.
	#[serde(flatten)]
	destination: super::map_schema::MapSchema,
	/// Transition details.
	#[serde(flatten)]
	transition: super::transition_schema::TransitionSchema,

}
