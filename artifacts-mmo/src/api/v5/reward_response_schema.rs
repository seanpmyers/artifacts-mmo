#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RewardResponseSchema {
	/// DropRateSchema
	data: super::drop_rate_schema::DropRateSchema,

}
