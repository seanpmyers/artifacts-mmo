#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GiveGoldReponseSchema {
	/// GiveGoldDataSchema
	data: super::give_gold_data_schema::GiveGoldDataSchema,

}
