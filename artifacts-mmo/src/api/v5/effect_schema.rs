#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EffectSchema {
	/// Code: The code of the effect. This is the effect's unique identifier (ID).
	code: String,
	/// Description: Description of the effect. This is a brief description of the effect.
	description: String,
	/// Name: Name of the effect.
	name: String,
	/// Subtype of the effect.
	#[serde(flatten)]
	subtype: super::effect_subtype::EffectSubtype,
	/// Type of the effect.
	#[serde(flatten)]
	EffectSchema_type: super::effect_type::EffectType,

}
