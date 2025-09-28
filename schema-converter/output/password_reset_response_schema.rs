#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct PasswordResetResponseSchema {
	/// Message: Success message.
	message: String,

}
