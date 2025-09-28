#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ValidationError {
	/// Location
	loc: Vec<serde_json::Value>,
	/// Message
	msg: String,
	/// Error Type
	ValidationError_type: String,

}
