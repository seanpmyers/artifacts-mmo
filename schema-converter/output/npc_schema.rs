#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct NPCSchema {
	/// Code: The code of the NPC. This is the NPC's unique identifier (ID).
	code: String,
	/// Description: Description of the NPC.
	description: String,
	/// Name: Name of the NPC.
	name: String,
	/// Type of the NPC.
	#[serde(flatten)]
	NPCSchema_type: NPCType,

}
