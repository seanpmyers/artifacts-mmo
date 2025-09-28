#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AddAccountSchema {
	/// Email: Your email.
	email: String,
	/// Password: Your password.
	password: String,
	/// Username: Your desired username.
	username: String,

}
