#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankGoldTransactionSchema {
	/// Bank details.
	#[serde(flatten)]
	bank: super::gold_schema::GoldSchema,
	/// Player details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,

}
