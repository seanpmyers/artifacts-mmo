#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MyCharactersListSchema {
	/// Data: List of your characters.
	data: Vec<super::character_schema::CharacterSchema>,

}
