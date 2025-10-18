#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CombatResultSchema {
	/// Character Results: Character results from combat.
	character_results: Vec<serde_json::Value>,
	/// Logs: Combat logs.
	logs: Vec<serde_json::Value>,
	/// Result: Combat result: 'win' or 'loss'.
	result: String,
	/// Turns: Number of turns the combat lasted.
	turns: i32,

}
