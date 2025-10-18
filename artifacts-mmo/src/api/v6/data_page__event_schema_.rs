#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DataPage_EventSchema_ {
	/// Data
	data: Vec<super::event_schema::EventSchema>,
	/// Page
	page: i32,
	/// Pages
	pages: i32,
	/// Size
	size: i32,
	/// Total
	total: i32,

}
