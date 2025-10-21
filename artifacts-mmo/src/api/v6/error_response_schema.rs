#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ErrorResponseSchema {
	error: super::error_schema::ErrorSchema,

}
