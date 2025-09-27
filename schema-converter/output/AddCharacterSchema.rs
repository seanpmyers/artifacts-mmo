#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AddCharacterSchema {
	/// Name: Your desired character name. It's unique and all players can see it.
	name: String,
	/// Your desired skin. Skins unlocked by default: 'men1', 'men2', 'men3', 'women1', 'women2', 'women3'.
	skin: #[serde(flatten)]
	CharacterSkin
,

}
