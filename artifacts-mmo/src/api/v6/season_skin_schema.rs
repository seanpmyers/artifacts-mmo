#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SeasonSkinSchema {
	/// Code: Skin code.
	code: String,
	/// Description: Skin description.
	description: String,
	/// Required Points: Required achievement points to earn the skin.
	required_points: i32,

}
