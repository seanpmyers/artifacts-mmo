#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangeSkinResponseSchema {
	/// ChangeSkinCharacterDataSchema
	data: ChangeSkinCharacterDataSchema,

}
