#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DeleteItemSchema {
	/// Player details.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details.
	cooldown: #[serde(flatten)]
	CooldownSchema
,
	/// Item details.
	item: #[serde(flatten)]
	SimpleItemSchema
,

}
