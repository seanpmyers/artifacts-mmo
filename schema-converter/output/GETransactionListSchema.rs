#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GETransactionListSchema {
	/// Character details.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details.
	cooldown: #[serde(flatten)]
	CooldownSchema
,
	/// Transaction details.
	order: #[serde(flatten)]
	GETransactionSchema
,

}
