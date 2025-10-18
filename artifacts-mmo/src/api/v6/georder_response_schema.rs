#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GEOrderResponseSchema {
	/// GEOrderSchema
	data: super::georder_schema::GEOrderSchema,

}
