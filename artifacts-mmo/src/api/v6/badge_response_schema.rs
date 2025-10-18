#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BadgeResponseSchema {
	/// BadgeSchema
	data: super::badge_schema::BadgeSchema,

}
