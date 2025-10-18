#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SpawnEventRequest {
	/// Code: Code of the event to spawn
	code: String,

}
