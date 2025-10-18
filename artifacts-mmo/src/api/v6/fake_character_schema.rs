#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct FakeCharacterSchema {
	/// Amulet Slot: Amulet slot item code.
	amulet_slot: String,
	/// Artifact1 Slot: Artifact 1 slot item code.
	artifact1_slot: String,
	/// Artifact2 Slot: Artifact 2 slot item code.
	artifact2_slot: String,
	/// Artifact3 Slot: Artifact 3 slot item code.
	artifact3_slot: String,
	/// Body Armor Slot: Body armor slot item code.
	body_armor_slot: String,
	/// Boots Slot: Boots slot item code.
	boots_slot: String,
	/// Helmet Slot: Helmet slot item code.
	helmet_slot: String,
	/// Leg Armor Slot: Leg armor slot item code.
	leg_armor_slot: String,
	/// Level: Character level.
	level: i32,
	/// Ring1 Slot: Ring 1 slot item code.
	ring1_slot: String,
	/// Ring2 Slot: Ring 2 slot item code.
	ring2_slot: String,
	/// Rune Slot: Rune slot item code.
	rune_slot: String,
	/// Shield Slot: Shield slot item code.
	shield_slot: String,
	/// Utility1 Slot: Utility 1 slot item code.
	utility1_slot: String,
	/// Utility1 Slot Quantity: Utility 1 quantity.
	utility1_slot_quantity: i32,
	/// Utility2 Slot: Utility 2 slot item code.
	utility2_slot: String,
	/// Utility2 Slot Quantity: Utility 2 quantity.
	utility2_slot_quantity: i32,
	/// Weapon Slot: Weapon slot item code.
	weapon_slot: String,

}
