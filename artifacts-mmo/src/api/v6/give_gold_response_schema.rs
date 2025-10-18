#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveGoldResponseSchema {
	/// GiveGoldDataSchema
	data: super::give_gold_data_schema::GiveGoldDataSchema,

}
