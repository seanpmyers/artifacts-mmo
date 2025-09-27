#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EquipSchema {
	/// Code: Item code.
	code: String,
	/// Quantity: Item quantity. Applicable to utilities only.
	quantity: i32,
	/// Item slot.
	slot: #[serde(flatten)]
	ItemSlot
,

}
