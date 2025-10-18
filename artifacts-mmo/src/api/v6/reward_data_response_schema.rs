#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RewardDataResponseSchema {
	/// RewardDataSchema
	data: super::reward_data_schema::RewardDataSchema,

}
