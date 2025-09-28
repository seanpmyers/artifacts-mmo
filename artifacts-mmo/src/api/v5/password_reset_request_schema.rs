#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct PasswordResetRequestSchema {
	/// Email: Your email address.
	email: String,

}
