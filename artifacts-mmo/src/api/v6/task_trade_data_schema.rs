#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskTradeDataSchema {
	/// Player details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Reward details.
	#[serde(flatten)]
	trade: super::task_trade_schema::TaskTradeSchema,

}
