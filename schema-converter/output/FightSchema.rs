#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct FightSchema {
	/// Drops: The items dropped from the fight.
	drops: TODO__Vec<>,
	/// Gold: The amount of gold gained from the fight.
	gold: i32,
	/// Logs: The fight logs.
	logs: TODO__Vec<>,
	/// The result of the fight.
	result: TODO__NOT A SINGLE TYPE,
	/// Turns: Numbers of the turns of the combat.
	turns: i32,
	/// Xp: The amount of xp gained from the fight.
	xp: i32,

}
