#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CharacterSchema {
	/// Account: Account name.
	account: String,
	/// Alchemy Level: Alchemy level.
	alchemy_level: i32,
	/// Alchemy Max Xp: Alchemy XP required to level up the skill.
	alchemy_max_xp: i32,
	/// Alchemy Xp: Alchemy XP.
	alchemy_xp: i32,
	/// Amulet Slot: Amulet slot.
	amulet_slot: String,
	/// Artifact1 Slot: Artifact 1 slot.
	artifact1_slot: String,
	/// Artifact2 Slot: Artifact 2 slot.
	artifact2_slot: String,
	/// Artifact3 Slot: Artifact 3 slot.
	artifact3_slot: String,
	/// Attack Air: Air attack.
	attack_air: i32,
	/// Attack Earth: Earth attack.
	attack_earth: i32,
	/// Attack Fire: Fire attack.
	attack_fire: i32,
	/// Attack Water: Water attack.
	attack_water: i32,
	/// Bag Slot: Bag slot.
	bag_slot: String,
	/// Body Armor Slot: Body armor slot.
	body_armor_slot: String,
	/// Boots Slot: Boots slot.
	boots_slot: String,
	/// Cooking Level: The current xp level of the Cooking skill.
	cooking_level: i32,
	/// Cooking Max Xp: Cooking XP required to level up the skill.
	cooking_max_xp: i32,
	/// Cooking Xp: Cooking XP.
	cooking_xp: i32,
	/// Cooldown: Cooldown in seconds.
	cooldown: i32,
	/// Cooldown Expiration: Datetime Cooldown expiration.
	cooldown_expiration: chrono::DateTime<chrono::Utc>,
	/// Critical Strike: % Critical strike. Critical strikes adds 50% extra damage to an attack (1.5x).
	critical_strike: i32,
	/// Dmg: % Damage. Damage increases your attack in all elements.
	dmg: i32,
	/// Dmg Air: % Air damage. Damage increases your air attack.
	dmg_air: i32,
	/// Dmg Earth: % Earth damage. Damage increases your earth attack.
	dmg_earth: i32,
	/// Dmg Fire: % Fire damage. Damage increases your fire attack.
	dmg_fire: i32,
	/// Dmg Water: % Water damage. Damage increases your water attack.
	dmg_water: i32,
	/// Effects: List of active effects on the character.
	effects: Vec<super::storage_effect_schema::StorageEffectSchema>,
	/// Fishing Level: Fishing level.
	fishing_level: i32,
	/// Fishing Max Xp: Fishing XP required to level up the skill.
	fishing_max_xp: i32,
	/// Fishing Xp: The current xp level of the Fishing skill.
	fishing_xp: i32,
	/// Gearcrafting Level: Gearcrafting level.
	gearcrafting_level: i32,
	/// Gearcrafting Max Xp: Gearcrafting XP required to level up the skill.
	gearcrafting_max_xp: i32,
	/// Gearcrafting Xp: The current xp level of the Gearcrafting skill.
	gearcrafting_xp: i32,
	/// Gold: The numbers of gold on this character.
	gold: i32,
	/// Haste: *Increase speed attack (reduce fight cooldown)
	haste: i32,
	/// Helmet Slot: Helmet slot.
	helmet_slot: String,
	/// Hp: Character actual HP.
	hp: i32,
	/// Initiative: Initiative determines turn order in combat. Higher initiative goes first.
	initiative: i32,
	/// Inventory: List of inventory slots.
	inventory: Vec<super::inventory_slot::InventorySlot>,
	/// Inventory Max Items: Inventory max items.
	inventory_max_items: i32,
	/// Jewelrycrafting Level: Jewelrycrafting level.
	jewelrycrafting_level: i32,
	/// Jewelrycrafting Max Xp: Jewelrycrafting XP required to level up the skill.
	jewelrycrafting_max_xp: i32,
	/// Jewelrycrafting Xp: The current xp level of the Jewelrycrafting skill.
	jewelrycrafting_xp: i32,
	/// Character current layer.
	#[serde(flatten)]
	layer: super::map_layer::MapLayer,
	/// Leg Armor Slot: Leg armor slot.
	leg_armor_slot: String,
	/// Level: Combat level.
	level: i32,
	/// Map Id: Character current map ID.
	map_id: i32,
	/// Max Hp: Character max HP.
	max_hp: i32,
	/// Max Xp: XP required to level up the character.
	max_xp: i32,
	/// Mining Level: Mining level.
	mining_level: i32,
	/// Mining Max Xp: Mining XP required to level up the skill.
	mining_max_xp: i32,
	/// Mining Xp: The current xp level of the Mining skill.
	mining_xp: i32,
	/// Name: Name of the character.
	name: String,
	/// Prospecting: Prospecting increases the chances of getting drops from fights and skills (1% extra per 10 PP).
	prospecting: i32,
	/// Res Air: % Air resistance. Reduces air attack.
	res_air: i32,
	/// Res Earth: % Earth resistance. Reduces earth attack.
	res_earth: i32,
	/// Res Fire: % Fire resistance. Reduces fire attack.
	res_fire: i32,
	/// Res Water: % Water resistance. Reduces water attack.
	res_water: i32,
	/// Ring1 Slot: Ring 1 slot.
	ring1_slot: String,
	/// Ring2 Slot: Ring 2 slot.
	ring2_slot: String,
	/// Rune Slot: Rune slot.
	rune_slot: String,
	/// Shield Slot: Shield slot.
	shield_slot: String,
	/// Character skin code.
	#[serde(flatten)]
	skin: super::character_skin::CharacterSkin,
	/// Speed: *Not available, on the roadmap. Character movement speed.
	speed: i32,
	/// Task: Task in progress.
	task: String,
	/// Task Progress: Task progression.
	task_progress: i32,
	/// Task Total: Task total objective.
	task_total: i32,
	/// Task Type: Task type.
	task_type: String,
	/// Threat: Threat level affects monster targeting in multi-character combat.
	threat: i32,
	/// Utility1 Slot: Utility 1 slot.
	utility1_slot: String,
	/// Utility1 Slot Quantity: Utility 1 quantity.
	utility1_slot_quantity: i32,
	/// Utility2 Slot: Utility 2 slot.
	utility2_slot: String,
	/// Utility2 Slot Quantity: Utility 2 quantity.
	utility2_slot_quantity: i32,
	/// Weapon Slot: Weapon slot.
	weapon_slot: String,
	/// Weaponcrafting Level: Weaponcrafting level.
	weaponcrafting_level: i32,
	/// Weaponcrafting Max Xp: Weaponcrafting XP required to level up the skill.
	weaponcrafting_max_xp: i32,
	/// Weaponcrafting Xp: The current xp level of the Weaponcrafting skill.
	weaponcrafting_xp: i32,
	/// Wisdom: Wisdom increases the amount of XP gained from fights and skills (1% extra per 10 wisdom).
	wisdom: i32,
	/// Woodcutting Level: Woodcutting level.
	woodcutting_level: i32,
	/// Woodcutting Max Xp: Woodcutting XP required to level up the skill.
	woodcutting_max_xp: i32,
	/// Woodcutting Xp: The current xp level of the Woodcutting skill.
	woodcutting_xp: i32,
	/// X: Character x coordinate.
	x: i32,
	/// Xp: The current xp level of the combat level.
	xp: i32,
	/// Y: Character y coordinate.
	y: i32,

}
