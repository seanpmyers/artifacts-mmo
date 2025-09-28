#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EventMapSchema {
	/// Skin: Map skin of the map
	skin: String,
	/// X: Position X of the map.
	x: i32,
	/// Y: Position Y of the map.
	y: i32,

}
