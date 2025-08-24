#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct LogSchema {
	/// Account: Account character.
	account: String,
	/// Character: Character name.
	character: String,
	/// Content: Content of action.
	content: TODO__NOT A SINGLE TYPE,
	/// Cooldown: Cooldown in seconds.
	cooldown: i32,
	/// Cooldown Expiration: Datetime of cooldown expiration.
	cooldown_expiration: TODO__NOT A SINGLE TYPE,
	/// Created At: Datetime of creation.
	created_at: chrono::DateTime<chrono::Utc>,
	/// Description: Description of action.
	description: String,
	/// Type of action.
	LogSchema_type: TODO__NOT A SINGLE TYPE,

}
