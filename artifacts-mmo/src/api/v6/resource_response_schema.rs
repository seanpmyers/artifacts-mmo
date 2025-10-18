#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ResourceResponseSchema {
	/// ResourceSchema
	data: super::resource_schema::ResourceSchema,

}
