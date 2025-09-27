#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EquipRequestSchema {
	/// Player details.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details.
	cooldown: #[serde(flatten)]
	CooldownSchema
,
	/// Item details.
	item: #[serde(flatten)]
	ItemSchema
,
	/// Item slot.
	slot: #[serde(flatten)]
	ItemSlot
,

}
