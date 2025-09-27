#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankGoldTransactionSchema {
	/// Bank details.
	bank: #[serde(flatten)]
	GoldSchema
,
	/// Player details.
	character: #[serde(flatten)]
	CharacterSchema
,
	/// Cooldown details.
	cooldown: #[serde(flatten)]
	CooldownSchema
,

}
