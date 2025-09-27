#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct UseItemSchema {
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
	ItemSchema
,

}
