#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveItemReponseSchema {
	/// GiveItemDataSchema
	data: GiveItemDataSchema,

}
