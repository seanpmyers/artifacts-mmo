#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SkillDataSchema {
	/// Player details.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: CooldownSchema,
	/// Craft details.
	#[serde(flatten)]
	details: SkillInfoSchema,

}
