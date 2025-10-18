#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterRestDataSchema {
	/// Character details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Hp Restored: The amount of HP restored.
	hp_restored: i32,

}
