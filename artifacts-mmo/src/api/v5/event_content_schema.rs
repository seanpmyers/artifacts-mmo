#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EventContentSchema {
	/// Code: Code content.
	code: String,
	/// Type of the event.
	#[serde(flatten)]
	EventContentSchema_type: MapContentType,

}
