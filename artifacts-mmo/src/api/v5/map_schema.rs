#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MapSchema {
	/// Content of the map.
	content: super::map_content_schema::MapContentSchema,
	/// Name: Name of the map.
	name: String,
	/// Skin: Skin of the map.
	skin: String,
	/// X: Position X of the map.
	x: i32,
	/// Y: Position Y of the map.
	y: i32,

}
