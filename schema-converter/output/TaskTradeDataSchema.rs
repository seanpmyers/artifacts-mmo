#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskTradeDataSchema {
	/// Player details.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details.
	cooldown: #[serde(flatten)]
	CooldownSchema
,
	/// Reward details.
	trade: #[serde(flatten)]
	TaskTradeSchema
,

}
