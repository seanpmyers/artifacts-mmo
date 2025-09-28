#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TaskCancelledSchema {
	/// Player details.
	#[serde(flatten)]
	character: CharacterSchema,
	/// Cooldown details.
	#[serde(flatten)]
	cooldown: CooldownSchema,

}
