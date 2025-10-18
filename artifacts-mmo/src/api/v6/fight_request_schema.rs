#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct FightRequestSchema {
	/// Participants: Optional list of additional character names to include in the fight (max 2 additional characters).
	participants: Vec<serde_json::Value>,

}
