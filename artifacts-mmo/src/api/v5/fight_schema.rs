#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct FightSchema {
	/// Drops: The items dropped from the fight.
	drops: Vec<super::drop_schema::DropSchema>,
	/// Gold: The amount of gold gained from the fight.
	gold: i32,
	/// Logs: The fight logs.
	logs: Vec<serde_json::Value>,
	/// The result of the fight.
	#[serde(flatten)]
	result: super::fight_result::FightResult,
	/// Turns: Numbers of the turns of the combat.
	turns: i32,
	/// Xp: The amount of xp gained from the fight.
	xp: i32,

}
