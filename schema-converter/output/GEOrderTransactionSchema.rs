#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GEOrderTransactionSchema {
	/// Character details.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details.
	cooldown: #[serde(flatten)]
	CooldownSchema
,
	/// Order details.
	order: #[serde(flatten)]
	GEOrderCreatedSchema
,

}
