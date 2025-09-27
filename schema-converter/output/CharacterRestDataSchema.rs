#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterRestDataSchema {
	/// Character details.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details
	cooldown: #[serde(flatten)]
	CooldownSchema
,
	/// Hp Restored: The amount of HP restored.
	hp_restored: i32,

}
