#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RewardDataSchema {
	/// Player details.
	#[serde(flatten)]
	character: super::character_schema::CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: super::cooldown_schema::CooldownSchema,
	/// Reward details.
	#[serde(flatten)]
	rewards: super::rewards_schema::RewardsSchema,

}
