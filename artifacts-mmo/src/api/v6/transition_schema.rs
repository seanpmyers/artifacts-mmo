#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TransitionSchema {
	/// Conditions: Conditions for the transition.
	conditions: Vec<super::condition_schema::ConditionSchema>,
	/// Layer of the destination.
	#[serde(flatten)]
	layer: super::map_layer::MapLayer,
	/// Map Id: ID of the destination map.
	map_id: i32,
	/// X: Position X of the destination.
	x: i32,
	/// Y: Position Y of the destination.
	y: i32,

}
