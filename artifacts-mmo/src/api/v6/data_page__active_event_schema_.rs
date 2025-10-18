#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DataPage_ActiveEventSchema_ {
	/// Data
	data: Vec<super::active_event_schema::ActiveEventSchema>,
	/// Page
	page: i32,
	/// Pages
	pages: i32,
	/// Size
	size: i32,
	/// Total
	total: i32,

}
