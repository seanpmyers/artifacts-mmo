#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EventContentSchema {
	/// Code: Code content.
	code: String,
	/// Type of the event.
	#[serde(flatten)]
	#[serde(rename = "type")]
	r#type: super::map_content_type::MapContentType,

}
