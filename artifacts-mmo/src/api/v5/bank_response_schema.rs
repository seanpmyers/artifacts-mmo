#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BankResponseSchema {
	/// BankSchema
	data: super::bank_schema::BankSchema,

}
