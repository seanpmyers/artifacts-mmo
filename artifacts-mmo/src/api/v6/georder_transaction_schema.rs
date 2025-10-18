#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GEOrderTransactionSchema {
	/// Character details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Order details.
	#[serde(flatten)]
	order: super::georder_created_schema::GEOrderCreatedSchema,

}
