#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankExtensionSchema {
	/// Price: Price of the bank extension.
	price: i32,

}
