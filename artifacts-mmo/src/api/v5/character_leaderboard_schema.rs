#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterLeaderboardSchema {
	/// Account: Account name.
	account: String,
	/// Alchemy Level: Alchemy level.
	alchemy_level: i32,
	/// Alchemy Total Xp: Alchemy total xp.
	alchemy_total_xp: i32,
	/// Cooking Level: Cooking level.
	cooking_level: i32,
	/// Cooking Total Xp: Cooking total xp.
	cooking_total_xp: i32,
	/// Fishing Level: Fishing level.
	fishing_level: i32,
	/// Fishing Total Xp: Fishing total xp.
	fishing_total_xp: i32,
	/// Gearcrafting Level: Gearcrafting level.
	gearcrafting_level: i32,
	/// Gearcrafting Total Xp: Gearcrafting total xp.
	gearcrafting_total_xp: i32,
	/// Gold: The numbers of gold on this character.
	gold: i32,
	/// Jewelrycrafting Level: Jewelrycrafting level.
	jewelrycrafting_level: i32,
	/// Jewelrycrafting Total Xp: Jewelrycrafting total xp.
	jewelrycrafting_total_xp: i32,
	/// Level: Combat level.
	level: i32,
	/// Mining Level: Mining level.
	mining_level: i32,
	/// Mining Total Xp: Mining total xp.
	mining_total_xp: i32,
	/// Name: Character name.
	name: String,
	/// Position: Position in the leaderboard.
	position: i32,
	/// Skin: Character skin code.
	skin: String,
	/// Member status.
	#[serde(flatten)]
	status: super::account_status::AccountStatus,
	/// Total Xp: Total XP of your character.
	total_xp: i32,
	/// Weaponcrafting Level: Weaponcrafting level.
	weaponcrafting_level: i32,
	/// Weaponcrafting Total Xp: Weaponcrafting total xp.
	weaponcrafting_total_xp: i32,
	/// Woodcutting Level: Woodcutting level.
	woodcutting_level: i32,
	/// Woodcutting Total Xp: Woodcutting total xp.
	woodcutting_total_xp: i32,

}
