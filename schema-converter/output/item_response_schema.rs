#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ItemResponseSchema {
	/// ItemSchema
	data: ItemSchema,

}
