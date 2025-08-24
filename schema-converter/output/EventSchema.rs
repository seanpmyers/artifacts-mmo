#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct EventSchema {
	/// Code: Code of the event.
	code: String,
	/// Content of the event.
	content: TODO__NOT A SINGLE TYPE,
	/// Duration: Duration in minutes.
	duration: i32,
	/// Maps: Map list of the event.
	maps: TODO__Vec<>,
	/// Name: Name of the event.
	name: String,
	/// Rate: Rate spawn of the event. (1/rate every minute)
	rate: i32,

}
