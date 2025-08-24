#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveItemsSchema {
	/// Character: Character name. The name of the character who will receive the items.
	character: String,
	/// Items: List of items to give
	items: TODO__Vec<>,

}
