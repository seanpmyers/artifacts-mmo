#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct MapResponseSchema {
	/// MapSchema
	data: MapSchema,

}
