#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterFightDataSchema {
	/// Player details.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: CooldownSchema,
	/// Fight details.
	#[serde(flatten)]
	fight: FightSchema,

}
