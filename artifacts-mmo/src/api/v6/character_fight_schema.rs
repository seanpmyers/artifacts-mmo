#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterFightSchema {
	/// Characters: Results for each character.
	characters: Vec<super::character_multi_fight_result_schema::CharacterMultiFightResultSchema>,
	/// Logs: The fight logs.
	logs: Vec<serde_json::Value>,
	/// Opponent: The code of the monster fought.
	opponent: String,
	/// The result of the fight.
	#[serde(flatten)]
	result: super::fight_result::FightResult,
	/// Turns: Numbers of the turns of the combat.
	turns: i32,

}
