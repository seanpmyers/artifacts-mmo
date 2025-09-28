#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ResourceSchema {
	/// Code: The code of the resource. This is the resource's unique identifier (ID).
	code: String,
	/// Drops: The drops of this resource.
	drops: Vec<DropRateSchema>,
	/// Level: The skill level required to gather this resource.
	level: i32,
	/// Name: The name of the resource
	name: String,
	/// Skill code: The skill required to gather this resource.
	#[serde(flatten)]
	skill: GatheringSkill,

}
