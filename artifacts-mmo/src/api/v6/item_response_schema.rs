#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ItemResponseSchema {
	/// ItemSchema
	data: super::item_schema::ItemSchema,

}
