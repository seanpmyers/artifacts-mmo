#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct UnequipSchema {
	/// Quantity: Item quantity. Applicable to utilities only.
	quantity: i32,
	/// Item slot.
	#[serde(flatten)]
	slot: ItemSlot,

}
