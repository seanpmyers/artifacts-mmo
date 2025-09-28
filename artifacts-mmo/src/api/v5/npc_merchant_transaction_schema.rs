#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NpcMerchantTransactionSchema {
	/// Character details.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: CooldownSchema,
	/// Transaction details.
	#[serde(flatten)]
	transaction: NpcItemTransactionSchema,

}
