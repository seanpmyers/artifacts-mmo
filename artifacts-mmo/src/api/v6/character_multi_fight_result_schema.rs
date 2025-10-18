#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterMultiFightResultSchema {
	/// Character Name: Name of the character.
	character_name: String,
	/// Drops: Items dropped for this character.
	drops: Vec<super::drop_schema::DropSchema>,
	/// Final Hp: Character's HP at the end of combat.
	final_hp: i32,
	/// Gold: Gold gained by this character.
	gold: i32,
	/// Xp: XP gained by this character.
	xp: i32,

}
