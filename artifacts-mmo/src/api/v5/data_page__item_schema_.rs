#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DataPage_ItemSchema_ {
	/// Data
	data: Vec<ItemSchema>,
	/// Page
	page: Option<f64>,
	/// Pages
	pages: Option<f64>,
	/// Size
	size: Option<f64>,
	/// Total
	total: Option<f64>,

}
