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
	#[serde(flatten)]
	map: MapSchema,
	/// Name: Name of the event.
	name: String,
	/// Previous map skin.
	#[serde(flatten)]
	previous_map: MapSchema,

}
