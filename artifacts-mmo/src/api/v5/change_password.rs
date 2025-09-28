#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangePassword {
	/// Current Password: Your password.
	current_password: String,
	/// New Password: New password.
	new_password: String,

}
