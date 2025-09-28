#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterRestDataSchema {
	/// Character details.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details
	#[serde(flatten)]
	cooldown: CooldownSchema,
	/// Hp Restored: The amount of HP restored.
	hp_restored: i32,

}
