#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MapContentSchema {
	/// Code: Code of the content.
	code: String,
	/// Type of the content.
	MapContentSchema_type: #[serde(flatten)]
	MapContentType
,

}
