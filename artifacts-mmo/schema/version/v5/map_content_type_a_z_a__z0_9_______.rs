#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MapContentType_a_zA_Z0_9_______ {
	#[default]
	Monster,
	Resource,
	Workshop,
	Bank,
	GrandExchange,
	TasksMaster,
	Npc,
}
