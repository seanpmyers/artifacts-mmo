#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct HTTPValidationError {
	/// Detail
	detail: Vec<ValidationError>,

}
