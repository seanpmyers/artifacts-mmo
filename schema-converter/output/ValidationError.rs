#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ValidationError {
	/// Location
	loc: TODO__Vec<>,
	/// Message
	msg: String,
	/// Error Type
	ValidationError_type: String,

}
