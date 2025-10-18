#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ErrorSchema {
	/// Error code
	code: i32,
	/// Additional error data (used primarily for validation errors)
	data: TODO_OBJECT_NOT_REFERENCE,
	/// Error message
	message: String,

}
