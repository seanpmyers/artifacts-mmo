#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AnnouncementSchema {
	/// Created At: Datetime of the announcement.
	created_at: chrono::DateTime<chrono::Utc>,
	/// Message: Announcement text.
	message: String,

}
