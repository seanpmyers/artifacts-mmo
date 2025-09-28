#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DataPage_EventSchema_ {
	/// Data
	data: Vec<EventSchema>,
	/// Page
	page: Option<f64>,
	/// Pages
	pages: Option<f64>,
	/// Size
	size: Option<f64>,
	/// Total
	total: Option<f64>,

}
