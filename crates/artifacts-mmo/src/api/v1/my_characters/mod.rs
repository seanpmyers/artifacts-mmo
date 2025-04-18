pub mod handler;

// use serde::{Deserialize, Serialize};
// use serde_json::Value;

// use crate::{
//     constants::api::ARTIFACTS_MMO_HOST,
//     http::{ApiEndpoint, HttpRequestMethod},
// };

// use super::{characters::Character, QueryParameters};

// pub const GET_MY_CHARACTERS: ApiEndpoint = ApiEndpoint {
//     host: ARTIFACTS_MMO_HOST,
//     path: "/my/characters",
//     http_request_method: HttpRequestMethod::GET,
// };
// pub const GET_CHARACTER_LOGS: ApiEndpoint = ApiEndpoint {
//     host: ARTIFACTS_MMO_HOST,
//     path: "/my/{name}/logs",
//     http_request_method: HttpRequestMethod::GET,
// };
// pub const GET_ALL_CHARACTER_LOGS: ApiEndpoint = ApiEndpoint {
//     host: ARTIFACTS_MMO_HOST,
//     path: "/my/logs",
//     http_request_method: HttpRequestMethod::GET,
// };

// pub const ACTION_MOVE: ApiEndpoint = ApiEndpoint {
//     host: ARTIFACTS_MMO_HOST,
//     path: "/my/{name}/action/move",
//     http_request_method: HttpRequestMethod::POST,
// };

// pub const ACTION_GATHERING: ApiEndpoint = ApiEndpoint {
//     host: ARTIFACTS_MMO_HOST,
//     path: "/my/{name}/action/gathering",
//     http_request_method: HttpRequestMethod::POST,
// };

// pub const ACTION_CRAFTING: ApiEndpoint = ApiEndpoint {
//     host: ARTIFACTS_MMO_HOST,
//     path: "/my/{name}/action/crafting",
//     http_request_method: HttpRequestMethod::POST,
// };

// pub const ACTION_GE_SELL_ITEM: ApiEndpoint = ApiEndpoint {
//     host: ARTIFACTS_MMO_HOST,
//     path: "/my/{name}/action/ge/sell",
//     http_request_method: HttpRequestMethod::POST,
// };

// #[derive(Debug, Serialize, Deserialize)]
// pub struct GetMyCharactersResponse {
//     pub data: Vec<Character>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionMoveRequest {
//     pub x: i32,
//     pub y: i32,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionMoveResponse {
//     pub data: ActionMove,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionMove {
//     pub cooldown: Cooldown,
//     pub destination: Destination,
//     pub character: Character,
// }

// impl QueryParameters for ActionMoveRequest {
//     fn get_path(parameter: String) -> String {
//         ACTION_MOVE.path.to_string().replace("{name}", &parameter)
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Cooldown {
//     pub total_seconds: i32,
//     pub remaining_seconds: i32,
//     pub expiration: String,
//     pub reason: CooldownReason,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum CooldownReason {
//     Movement,
//     Fight,
//     Crafting,
//     Gathering,
//     BuyGe,
//     SellGe,
//     DeleteItem,
//     DepositBank,
//     WithdrawBank,
//     Equip,
//     Unequip,
//     Task,
//     Recycling,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Destination {
//     pub name: String,
//     pub x: i32,
//     pub y: i32,
//     pub content: ContentVariant,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum ContentVariant {
//     Content(Content),
//     #[serde(untagged)]
//     Null(Value),
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Content {
//     pub code: Option<String>,
//     pub resource: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionGatheringRequest {
//     pub name: String,
// }

// impl QueryParameters for ActionGatheringRequest {
//     fn get_path(parameter: String) -> String {
//         ACTION_GATHERING
//             .path
//             .to_string()
//             .replace("{name}", &parameter)
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionGatheringResponse {
//     pub data: ActionGathering,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionGathering {
//     pub cooldown: Cooldown,
//     pub character: Character,
//     pub details: SkillInfoSchema,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SkillInfoSchema {
//     pub xp: f32,
//     pub items: Vec<DropSchema>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct DropSchema {
//     pub code: String,
//     pub quantity: f32,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionCraftingRequest {
//     pub code: String,
//     pub quantity: i32,
// }

// impl QueryParameters for ActionCraftingRequest {
//     fn get_path(parameter: String) -> String {
//         ACTION_CRAFTING
//             .path
//             .to_string()
//             .replace("{name}", &parameter)
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionCraftingResponse {
//     pub data: ActionCrafting,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionCrafting {
//     pub cooldown: Cooldown,
//     pub character: Character,
//     pub details: SkillInfoSchema,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionGeSellItemRequest {
//     pub code: String,
//     pub quantity: i32,
//     pub price: f32,
// }

// impl QueryParameters for ActionGeSellItemRequest {
//     fn get_path(parameter: String) -> String {
//         ACTION_GE_SELL_ITEM
//             .path
//             .to_string()
//             .replace("{name}", &parameter)
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ActionGeSellItemResponse {
//     pub data: GETransactionListSchema,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct GETransactionListSchema {
//     pub cooldown: Cooldown,
//     pub character: Character,
//     pub transaction: GETransactionSchema,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct GETransactionSchema {
//     pub code: String,
//     pub quantity: i32,
//     pub price: f32,
//     pub total_price: f32,
// }
