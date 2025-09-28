#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DeleteItemResponseSchema {
	/// DeleteItemSchema
	data: DeleteItemSchema,

}
