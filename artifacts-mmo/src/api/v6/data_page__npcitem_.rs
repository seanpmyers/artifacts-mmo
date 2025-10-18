#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DataPage_NPCItem_ {
	/// Data
	data: Vec<super::npc_item::NPCItem>,
	/// Page
	page: i32,
	/// Pages
	pages: i32,
	/// Size
	size: i32,
	/// Total
	total: i32,

}
