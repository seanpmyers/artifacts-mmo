#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RecyclingResponseSchema {
	/// RecyclingDataSchema
	data: super::recycling_data_schema::RecyclingDataSchema,

}
