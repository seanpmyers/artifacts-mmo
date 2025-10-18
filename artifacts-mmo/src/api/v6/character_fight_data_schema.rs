#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterFightDataSchema {
	/// Characters: All characters involved.
	characters: Vec<super::character_schema::CharacterSchema>,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Character fight details.
	#[serde(flatten)]
	fight: super::character_fight_schema::CharacterFightSchema,

}
