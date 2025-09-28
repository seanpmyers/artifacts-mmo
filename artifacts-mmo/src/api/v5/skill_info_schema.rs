#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SkillInfoSchema {
	/// Items: Objects received.
	items: Vec<super::drop_schema::DropSchema>,
	/// Xp: The amount of xp gained.
	xp: i32,

}
