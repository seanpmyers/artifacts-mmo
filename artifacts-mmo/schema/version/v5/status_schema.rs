#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct StatusSchema {
	/// Announcements: Server announcements.
	announcements: Vec<AnnouncementSchema>,
	/// Characters Online: Characters online.
	characters_online: i32,
	/// Max Level: Maximum level.
	max_level: i32,
	/// Max Skill Level: Maximum skill level.
	max_skill_level: i32,
	/// Rate Limits: Rate limits.
	rate_limits: Vec<RateLimitSchema>,
	/// Current season details.
	#[serde(flatten)]
	season: SeasonSchema,
	/// Server Time: Server time.
	server_time: chrono::DateTime<chrono::Utc>,
	/// Version: Game version.
	version: String,

}
