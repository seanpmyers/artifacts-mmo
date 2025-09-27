#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MonsterSchema {
	/// Attack Air: Monster air attack.
	attack_air: i32,
	/// Attack Earth: Monster earth attack.
	attack_earth: i32,
	/// Attack Fire: Monster fire attack.
	attack_fire: i32,
	/// Attack Water: Monster water attack.
	attack_water: i32,
	/// Code: The code of the monster. This is the monster's unique identifier (ID).
	code: String,
	/// Critical Strike: Monster % critical strike.
	critical_strike: i32,
	/// Drops: Monster drops. This is a list of items that the monster drops after killing the monster. 
	drops: Vec<DropRateSchema>,
	/// Effects: List of effects.
	effects: Vec<SimpleEffectSchema>,
	/// Hp: Monster hit points.
	hp: i32,
	/// Level: Monster level.
	level: i32,
	/// Max Gold: Monster maximum gold drop. 
	max_gold: i32,
	/// Min Gold: Monster minimum gold drop. 
	min_gold: i32,
	/// Name: Name of the monster.
	name: String,
	/// Res Air: Monster % air resistance.
	res_air: i32,
	/// Res Earth: Monster % earth resistance.
	res_earth: i32,
	/// Res Fire: Monster % fire resistance.
	res_fire: i32,
	/// Res Water: Monster % water resistance.
	res_water: i32,

}
