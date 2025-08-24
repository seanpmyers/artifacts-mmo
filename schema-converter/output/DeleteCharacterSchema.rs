#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DeleteCharacterSchema {
	/// Name: Character name.
	name: String,

}
