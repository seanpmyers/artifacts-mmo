#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EffectResponseSchema {
	/// EffectSchema
	data: super::effect_schema::EffectSchema,

}
