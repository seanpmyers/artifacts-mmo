#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveItemResponseSchema {
	/// GiveItemDataSchema
	data: super::give_item_data_schema::GiveItemDataSchema,

}
