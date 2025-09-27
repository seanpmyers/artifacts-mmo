#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EffectSchema {
	/// Code: The code of the effect. This is the effect's unique identifier (ID).
	code: String,
	/// Description: Description of the effect. This is a brief description of the effect.
	description: String,
	/// Name: Name of the effect.
	name: String,
	/// Subtype of the effect.
	subtype: #[serde(flatten)]
	EffectSubtype
,
	/// Type of the effect.
	EffectSchema_type: #[serde(flatten)]
	EffectType
,

}
