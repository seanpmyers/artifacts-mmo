#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CombatSimulationRequestSchema {
	/// Characters: List of fake characters (1-3).
	characters: Vec<super::fake_character_schema::FakeCharacterSchema>,
	/// Iterations: Number of combat iterations to simulate.
	iterations: i32,
	/// Monster: Monster code to fight against.
	monster: String,

}
