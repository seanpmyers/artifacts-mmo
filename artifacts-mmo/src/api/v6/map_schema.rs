#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MapSchema {
	/// Access information for the map
	#[serde(flatten)]
	access: super::access_schema::AccessSchema,
	/// Interactions available on this map.
	#[serde(flatten)]
	interactions: super::interaction_schema::InteractionSchema,
	/// Layer of the map.
	#[serde(flatten)]
	layer: super::map_layer::MapLayer,
	/// Map Id: ID of the map.
	map_id: i32,
	/// Name: Name of the map.
	name: String,
	/// Skin: Skin of the map.
	skin: String,
	/// X: Position X of the map.
	x: i32,
	/// Y: Position Y of the map.
	y: i32,

}
