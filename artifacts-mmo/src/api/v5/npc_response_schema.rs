#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NPCResponseSchema {
	/// NPCSchema
	data: super::npc_schema::NPCSchema,

}
