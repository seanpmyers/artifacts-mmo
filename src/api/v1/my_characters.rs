use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{ApiEndpoint, HttpRequestMethod},
};

use super::{
    characters::{Character, Skin},
    QueryParameters,
};

pub const GET_MY_CHARACTERS: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/my/characters",
    http_request_method: HttpRequestMethod::GET,
};
pub const GET_CHARACTER_LOGS: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/my/{name}/logs",
    http_request_method: HttpRequestMethod::GET,
};
pub const GET_ALL_CHARACTER_LOGS: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/my/logs",
    http_request_method: HttpRequestMethod::GET,
};

pub const ACTION_MOVE: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/my/{name}/action/move",
    http_request_method: HttpRequestMethod::POST,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMyCharactersResponse {
    pub data: Vec<MyCharacters>,
    pub total: i32,
    pub page: i32,
    pub size: i32,
    pub pages: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyCharacters {
    pub name: String,
    pub skin: Skin,
    pub level: i32,
    pub xp: i32,
    pub max_xp: i32,
    pub total_xp: i32,
    pub gold: i32,
    pub speed: i32,
    pub mining_level: i32,
    pub mining_xp: i32,
    pub mining_max_xp: i32,
    pub woodcutting_level: i32,
    pub woodcutting_xp: i32,
    pub woodcutting_max_xp: i32,
    pub fishing_level: i32,
    pub fishing_xp: i32,
    pub fishing_max_xp: i32,
    pub cooking_level: i32,
    pub cooking_xp: i32,
    pub cooking_max_xp: i32,
    pub weaponcrafting_level: i32,
    pub weaponcrafting_xp: i32,
    pub weaponcrafting_max_xp: i32,
    pub gearcrafting_level: i32,
    pub gearcrafting_xp: i32,
    pub gearcrafting_max_xp: i32,
    pub jewelrycrafting_level: i32,
    pub jewelrycrafting_xp: i32,
    pub jewelrycrafting_max_xp: i32,
    pub hp: i32,
    pub critical_strike: i32,
    pub stamina: i32,
    pub attack_fire: i32,
    pub attack_earth: i32,
    pub attack_water: i32,
    pub attack_air: i32,
    pub dmg_fire: f32,
    pub dmg_earth: f32,
    pub dmg_water: f32,
    pub dmg_air: f32,
    pub res_fire: f32,
    pub res_earth: f32,
    pub res_water: f32,
    pub res_air: f32,
    pub x: i32,
    pub y: i32,
    pub cooldown: i32,
    pub cooldown_expiration: String,
    pub weapon_slot: String,
    pub shield_slot: String,
    pub helmet_slot: String,
    pub body_armor_slot: String,
    pub leg_armor_slot: String,
    pub boots_slot: String,
    pub ring1_slot: String,
    pub ring2_slot: String,
    pub amulet_slot: String,
    pub artifact1_slot: String,
    pub artifact2_slot: String,
    pub artifact3_slot: String,
    pub consumable1_slot: String,
    pub consumable1_slot_quantity: i32,
    pub consumable2_slot: String,
    pub consumable2_slot_quantity: i32,
    pub inventory_slot1: String,
    pub inventory_slot1_quantity: i32,
    pub inventory_slot2: String,
    pub inventory_slot2_quantity: i32,
    pub inventory_slot3: String,
    pub inventory_slot3_quantity: i32,
    pub inventory_slot4: String,
    pub inventory_slot4_quantity: i32,
    pub inventory_slot5: String,
    pub inventory_slot5_quantity: i32,
    pub inventory_slot6: String,
    pub inventory_slot6_quantity: i32,
    pub inventory_slot7: String,
    pub inventory_slot7_quantity: i32,
    pub inventory_slot8: String,
    pub inventory_slot8_quantity: i32,
    pub inventory_slot9: String,
    pub inventory_slot9_quantity: i32,
    pub inventory_slot10: String,
    pub inventory_slot10_quantity: i32,
    pub inventory_slot11: String,
    pub inventory_slot11_quantity: i32,
    pub inventory_slot12: String,
    pub inventory_slot12_quantity: i32,
    pub inventory_slot13: String,
    pub inventory_slot13_quantity: i32,
    pub inventory_slot14: String,
    pub inventory_slot14_quantity: i32,
    pub inventory_slot15: String,
    pub inventory_slot15_quantity: i32,
    pub inventory_slot16: String,
    pub inventory_slot16_quantity: i32,
    pub inventory_slot17: String,
    pub inventory_slot17_quantity: i32,
    pub inventory_slot18: String,
    pub inventory_slot18_quantity: i32,
    pub inventory_slot19: String,
    pub inventory_slot19_quantity: i32,
    pub inventory_slot20: String,
    pub inventory_slot20_quantity: i32,
    pub inventory_max_items: i32,
    pub task: String,
    pub task_type: String,
    pub task_progress: i32,
    pub task_total: i32,
    pub account: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionMoveRequest {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionMoveResponse {
    data: ActionMove,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionMove {
    pub cooldown: Cooldown,
    pub destination: Destination,
    pub character: Character,
}

impl QueryParameters for ActionMove {
    fn get_path(parameter: String) -> String {
        ACTION_MOVE.path.to_string().replace("{name}", &parameter)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    pub total_seconds: i32,
    pub remaining_seconds: i32,
    pub expiration: String,
    pub reason: CooldownReason,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CooldownReason {
    Movement,
    Fight,
    Crafting,
    Gathering,
    BuyGe,
    SellGe,
    DeleteItem,
    DepositBank,
    WithdrawBank,
    Equip,
    Unequip,
    Task,
    Recycling,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Destination {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub content: ContentVariant,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContentVariant {
    Content(Content),
    #[serde(untagged)]
    Null(Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub code: Option<String>,
    pub resource: Option<String>,
}
