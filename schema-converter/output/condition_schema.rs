#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ConditionSchema {
	/// Code: Condition code.
	code: String,
	/// Condition operator.
	#[serde(flatten)]
	operator: ConditionOperator,
	/// Value: Condition value.
	value: i32,

}
