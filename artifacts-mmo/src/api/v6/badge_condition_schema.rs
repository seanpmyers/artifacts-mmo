#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BadgeConditionSchema {
	/// Code: Code of the condition.
	code: String,
	/// Quantity: Quantity of the condition (if any).
	quantity: i32,

}
