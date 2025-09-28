#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EquipRequestSchema {
	/// Player details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Item details.
	#[serde(flatten)]
	item: super::item_schema::ItemSchema,
	/// Item slot.
	#[serde(flatten)]
	slot: super::item_slot::ItemSlot,

}
