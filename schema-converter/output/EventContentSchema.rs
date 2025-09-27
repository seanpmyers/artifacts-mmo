#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EventContentSchema {
	/// Code: Code content.
	code: String,
	/// Type of the event.
	EventContentSchema_type: #[serde(flatten)]
	MapContentType
,

}
