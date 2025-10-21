#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ErrorSchema {
	/// Error code
	code: i32,
	/// Additional error data (used primarily for validation errors)
	data: serde_json::Value,
	/// Error message
	message: String,

}
