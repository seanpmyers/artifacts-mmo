#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DataPage_ActiveEventSchema_ {
	/// Data
	data: Vec<ActiveEventSchema>,
	/// Page
	page: Option<f64>,
	/// Pages
	pages: Option<f64>,
	/// Size
	size: Option<f64>,
	/// Total
	total: Option<f64>,

}
