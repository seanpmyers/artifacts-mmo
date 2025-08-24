#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TokenResponseSchema {
	/// Token
	token: String,

}
