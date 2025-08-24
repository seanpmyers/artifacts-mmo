#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DestinationSchema {
	/// X: The x coordinate of the destination.
	x: i32,
	/// Y: The y coordinate of the destination.
	y: i32,

}
