#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AccessSchema {
	/// Conditions: Access conditions for the map
	conditions: Vec<super::condition_schema::ConditionSchema>,
	/// Map access type determining movement and accessibility
	#[serde(flatten)]
	#[serde(rename = "type")]
	r#type: super::map_access_type::MapAccessType,

}
