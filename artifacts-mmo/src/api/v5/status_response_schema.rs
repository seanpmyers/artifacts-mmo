#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct StatusResponseSchema {
	/// StatusSchema
	data: super::status_schema::StatusSchema,

}
