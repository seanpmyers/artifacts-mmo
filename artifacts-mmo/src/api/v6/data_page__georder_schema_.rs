#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DataPage_GEOrderSchema_ {
	/// Data
	data: Vec<super::georder_schema::GEOrderSchema>,
	/// Page
	page: i32,
	/// Pages
	pages: i32,
	/// Size
	size: i32,
	/// Total
	total: i32,

}
