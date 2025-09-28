#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GEOrderReponseSchema {
	/// GEOrderSchema
	data: super::georder_schema::GEOrderSchema,

}
