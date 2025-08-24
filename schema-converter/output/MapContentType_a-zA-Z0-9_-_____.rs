#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MapContentType_a-zA-Z0-9_-_____ {
	#[default]
	Monster,
	Resource,
	Workshop,
	Bank,
	GrandExchange,
	TasksMaster,
	Npc,
}
