#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct PasswordResetConfirmSchema {
	/// New Password: Your new password.
	new_password: String,
	/// Token: Password reset token.
	token: String,

}
