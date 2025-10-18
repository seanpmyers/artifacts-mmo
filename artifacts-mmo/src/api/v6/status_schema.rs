#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct StatusSchema {
	/// Announcements: Server announcements.
	announcements: Vec<super::announcement_schema::AnnouncementSchema>,
	/// Characters Online: Characters online.
	characters_online: i32,
	/// Max Level: Maximum level.
	max_level: i32,
	/// Max Skill Level: Maximum skill level.
	max_skill_level: i32,
	/// Rate Limits: Rate limits.
	rate_limits: Vec<super::rate_limit_schema::RateLimitSchema>,
	/// Current season details.
	#[serde(flatten)]
	season: super::season_schema::SeasonSchema,
	/// Server Time: Server time.
	server_time: chrono::DateTime<chrono::Utc>,
	/// Version: Game version.
	version: String,

}
