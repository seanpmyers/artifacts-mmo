#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NPCItem {
	/// Buy Price: Price to buy the item.
	buy_price: i32,
	/// Code: The code of the NPC. This is the NPC's unique identifier (ID).
	code: String,
	/// Currency: Currency used to buy/sell the item. If it's not gold, it's the item code.
	currency: String,
	/// Npc: Code of the NPC that sells/buys the item.
	npc: String,
	/// Sell Price: Price to sell the item.
	sell_price: i32,

}
