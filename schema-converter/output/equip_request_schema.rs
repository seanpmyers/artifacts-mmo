#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EquipRequestSchema {
	/// Player details.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: CooldownSchema,
	/// Item details.
	#[serde(flatten)]
	item: ItemSchema,
	/// Item slot.
	#[serde(flatten)]
	slot: ItemSlot,

}
