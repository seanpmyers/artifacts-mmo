#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeSkinCharacterDataSchema {
	/// Player details.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: CooldownSchema,
	/// Skin: Craft details.
	skin: String,

}
