#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct StorageEffectSchema {
	/// Code: Effect code.
	code: String,
	/// Value: Effect value.
	value: i32,

}
