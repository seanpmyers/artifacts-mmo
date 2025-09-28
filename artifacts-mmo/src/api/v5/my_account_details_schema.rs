#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MyAccountDetailsSchema {
	/// MyAccountDetails
	data: super::my_account_details::MyAccountDetails,

}
