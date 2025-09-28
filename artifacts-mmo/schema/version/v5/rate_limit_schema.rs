#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RateLimitSchema {
	/// Type: Type of rate limit.
	RateLimitSchema_type: String,
	/// Value: Value of the rate limit.
	value: String,

}
