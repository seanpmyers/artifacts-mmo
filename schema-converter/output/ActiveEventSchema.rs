#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActiveEventSchema {
	/// Code: Code of the event.
	code: String,
	/// Created At: Start datetime.
	created_at: chrono::DateTime<chrono::Utc>,
	/// Duration: Duration in minutes.
	duration: i32,
	/// Expiration: Expiration datetime.
	expiration: chrono::DateTime<chrono::Utc>,
	/// Map of the event.
	map: TODO__NOT A SINGLE TYPE,
	/// Name: Name of the event.
	name: String,
	/// Previous map skin.
	previous_map: TODO__NOT A SINGLE TYPE,

}
