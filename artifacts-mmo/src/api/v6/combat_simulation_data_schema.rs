#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CombatSimulationDataSchema {
	/// Losses: Total number of defeats.
	losses: i32,
	/// Results: Results from each combat iteration.
	results: Vec<super::combat_result_schema::CombatResultSchema>,
	/// Winrate: Win rate percentage (0-100).
	winrate: f32,
	/// Wins: Total number of victories.
	wins: i32,

}
