#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct UseItemResponseSchema {
	/// UseItemSchema
	data: super::use_item_schema::UseItemSchema,

}
