#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct InteractionSchema {
	/// MapContentSchema
	content: super::map_content_schema::MapContentSchema,
	/// TransitionSchema
	transition: super::transition_schema::TransitionSchema,

}
