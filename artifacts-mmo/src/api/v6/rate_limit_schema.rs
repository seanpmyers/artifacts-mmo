#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct RateLimitSchema {
	/// Type: Type of rate limit.
	#[serde(rename = "type")]
	r#type: String,
	/// Value: Value of the rate limit.
	value: String,

}
