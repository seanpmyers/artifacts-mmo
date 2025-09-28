#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SimpleEffectSchema {
	/// Code: Effect code.
	code: String,
	/// Description: Description of the effect.
	description: String,
	/// Value: Effect value.
	value: i32,

}
