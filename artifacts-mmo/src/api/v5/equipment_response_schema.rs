#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EquipmentResponseSchema {
	/// EquipRequestSchema
	data: super::equip_request_schema::EquipRequestSchema,

}
