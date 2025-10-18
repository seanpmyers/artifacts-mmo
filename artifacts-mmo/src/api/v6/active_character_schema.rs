#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActiveCharacterSchema {
	/// Account: Account name.
	account: String,
	/// Character current layer.
	#[serde(flatten)]
	layer: super::map_layer::MapLayer,
	/// Map Id: Character current map ID.
	map_id: i32,
	/// Name: Name of the character.
	name: String,
	/// Character skin code.
	#[serde(flatten)]
	skin: super::character_skin::CharacterSkin,
	/// X: Character x coordinate.
	x: i32,
	/// Y: Character y coordinate.
	y: i32,

}
