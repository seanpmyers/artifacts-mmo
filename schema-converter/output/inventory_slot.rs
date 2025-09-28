#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct InventorySlot {
	/// Code: Item code.
	code: String,
	/// Quantity: Quantity in the slot.
	quantity: i32,
	/// Slot: Inventory slot identifier.
	slot: i32,

}
