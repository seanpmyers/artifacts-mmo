#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RewardsSchema {
	/// Gold: Gold rewards.
	gold: i32,
	/// Items: Items rewards.
	items: Vec<SimpleItemSchema>,

}
