#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AddAccountSchema {
	/// Email: Your email.
	email: String__TODO__email,
	/// Password: Your password.
	password: String,
	/// Username: Your desired username.
	username: String,

}
