#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CooldownSchema {
	/// Expiration: The expiration of the cooldown.
	expiration: chrono::DateTime<chrono::Utc>,
	/// The reason of the cooldown.
	#[serde(flatten)]
	reason: super::action_type::ActionType,
	/// Remaining Seconds: The remaining seconds of the cooldown.
	remaining_seconds: i32,
	/// Started At: The start of the cooldown.
	started_at: chrono::DateTime<chrono::Utc>,
	/// Total Seconds: The total seconds of the cooldown.
	total_seconds: i32,

}
