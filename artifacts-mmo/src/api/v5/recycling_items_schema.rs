#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RecyclingItemsSchema {
	/// Items: Objects received.
	items: Vec<super::drop_schema::DropSchema>,

}
