#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DataPage_GeOrderHistorySchema_ {
	/// Data
	data: Vec<super::ge_order_history_schema::GeOrderHistorySchema>,
	/// Page
	page: i32,
	/// Pages
	pages: i32,
	/// Size
	size: i32,
	/// Total
	total: i32,

}
