#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActiveEventResponseSchema {
	/// ActiveEventSchema
	data: super::active_event_schema::ActiveEventSchema,

}
