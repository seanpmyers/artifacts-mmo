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

pub const ACTION_GATHERING: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/my/{name}/action/gathering",
    http_request_method: HttpRequestMethod::POST,
};

pub const ACTION_CRAFTING: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/my/{name}/action/crafting",
    http_request_method: HttpRequestMethod::POST,
};

pub const ACTION_GE_SELL_ITEM: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/my/{name}/action/ge/sell",
    http_request_method: HttpRequestMethod::POST,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMyCharactersResponse {
    pub data: Vec<MyCharacters>,
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

impl MyCharacters {
    pub fn to_character(&self) -> Character {
        Character {
            name: self.name.clone(),
            skin: self.skin,
            level: self.level,
            xp: self.xp,
            max_xp: self.max_xp,
            total_xp: self.total_xp,
            gold: self.gold,
            speed: self.speed,
            mining_level: self.mining_level,
            mining_xp: self.mining_xp,
            mining_max_xp: self.mining_max_xp,
            woodcutting_level: self.woodcutting_level,
            woodcutting_xp: self.woodcutting_xp,
            woodcutting_max_xp: self.woodcutting_max_xp,
            fishing_level: self.fishing_level,
            fishing_xp: self.fishing_xp,
            fishing_max_xp: self.fishing_max_xp,
            cooking_level: self.cooking_level,
            cooking_xp: self.cooking_xp,
            cooking_max_xp: self.cooking_max_xp,
            weaponcrafting_level: self.weaponcrafting_level,
            weaponcrafting_xp: self.weaponcrafting_xp,
            weaponcrafting_max_xp: self.weaponcrafting_max_xp,
            gearcrafting_level: self.gearcrafting_level,
            gearcrafting_xp: self.gearcrafting_xp,
            gearcrafting_max_xp: self.gearcrafting_max_xp,
            jewelrycrafting_level: self.jewelrycrafting_level,
            jewelrycrafting_max_xp: self.jewelrycrafting_max_xp,
            jewelrycrafting_xp: self.jewelrycrafting_xp,
            hp: self.hp,
            critical_strike: self.critical_strike,
            stamina: self.stamina,
            attack_fire: self.attack_fire,
            attack_earth: self.attack_earth,
            attack_water: self.attack_water,
            attack_air: self.attack_air,
            dmg_fire: self.dmg_fire,
            dmg_earth: self.dmg_earth,
            dmg_water: self.dmg_water,
            dmg_air: self.dmg_air,
            res_fire: self.res_fire,
            res_earth: self.res_earth,
            res_water: self.res_water,
            res_air: self.res_air,
            x: self.x,
            y: self.y,
            cooldown: self.cooldown,
            cooldown_expiration: self.cooldown_expiration.clone(),
            weapon_slot: self.weapon_slot.clone(),
            shield_slot: self.shield_slot.clone(),
            helmet_slot: self.helmet_slot.clone(),
            body_armor_slot: self.body_armor_slot.clone(),
            leg_armor_slot: self.leg_armor_slot.clone(),
            boots_slot: self.boots_slot.clone(),
            ring1_slot: self.ring1_slot.clone(),
            ring2_slot: self.ring2_slot.clone(),
            amulet_slot: self.amulet_slot.clone(),
            artifact1_slot: self.artifact1_slot.clone(),
            artifact2_slot: self.artifact2_slot.clone(),
            artifact3_slot: self.artifact3_slot.clone(),
            consumable1_slot: self.consumable1_slot.clone(),
            consumable1_slot_quantity: self.consumable1_slot_quantity,
            consumable2_slot: self.consumable2_slot.clone(),
            consumable2_slot_quantity: self.consumable2_slot_quantity,
            inventory_slot1: self.inventory_slot1.clone(),
            inventory_slot1_quantity: self.inventory_slot1_quantity,
            inventory_slot2: self.inventory_slot2.clone(),
            inventory_slot2_quantity: self.inventory_slot2_quantity,
            inventory_slot3: self.inventory_slot3.clone(),
            inventory_slot3_quantity: self.inventory_slot3_quantity,
            inventory_slot4: self.inventory_slot4.clone(),
            inventory_slot4_quantity: self.inventory_slot4_quantity,
            inventory_slot5: self.inventory_slot5.clone(),
            inventory_slot5_quantity: self.inventory_slot5_quantity,
            inventory_slot6: self.inventory_slot6.clone(),
            inventory_slot6_quantity: self.inventory_slot6_quantity,
            inventory_slot7: self.inventory_slot7.clone(),
            inventory_slot7_quantity: self.inventory_slot7_quantity,
            inventory_slot8: self.inventory_slot8.clone(),
            inventory_slot8_quantity: self.inventory_slot8_quantity,
            inventory_slot9: self.inventory_slot9.clone(),
            inventory_slot9_quantity: self.inventory_slot9_quantity,
            inventory_slot10: self.inventory_slot10.clone(),
            inventory_slot10_quantity: self.inventory_slot10_quantity,
            inventory_slot11: self.inventory_slot11.clone(),
            inventory_slot11_quantity: self.inventory_slot11_quantity,
            inventory_slot12: self.inventory_slot12.clone(),
            inventory_slot12_quantity: self.inventory_slot12_quantity,
            inventory_slot13: self.inventory_slot13.clone(),
            inventory_slot13_quantity: self.inventory_slot13_quantity,
            inventory_slot14: self.inventory_slot14.clone(),
            inventory_slot14_quantity: self.inventory_slot14_quantity,
            inventory_slot15: self.inventory_slot15.clone(),
            inventory_slot15_quantity: self.inventory_slot15_quantity,
            inventory_slot16: self.inventory_slot16.clone(),
            inventory_slot16_quantity: self.inventory_slot16_quantity,
            inventory_slot17: self.inventory_slot17.clone(),
            inventory_slot17_quantity: self.inventory_slot17_quantity,
            inventory_slot18: self.inventory_slot18.clone(),
            inventory_slot18_quantity: self.inventory_slot18_quantity,
            inventory_slot19: self.inventory_slot19.clone(),
            inventory_slot19_quantity: self.inventory_slot19_quantity,
            inventory_slot20: self.inventory_slot20.clone(),
            inventory_slot20_quantity: self.inventory_slot20_quantity,
            inventory_max_items: self.inventory_max_items,
            task: self.task.clone(),
            task_type: self.task_type.clone(),
            task_progress: self.task_progress,
            task_total: self.task_total,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionMoveRequest {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionMoveResponse {
    pub data: ActionMove,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionMove {
    pub cooldown: Cooldown,
    pub destination: Destination,
    pub character: Character,
}

impl QueryParameters for ActionMoveRequest {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionGatheringRequest {
    pub name: String,
}

impl QueryParameters for ActionGatheringRequest {
    fn get_path(parameter: String) -> String {
        ACTION_GATHERING
            .path
            .to_string()
            .replace("{name}", &parameter)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionGatheringResponse {
    pub data: ActionGathering,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionGathering {
    pub cooldown: Cooldown,
    pub character: Character,
    pub details: SkillInfoSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillInfoSchema {
    pub xp: f32,
    pub items: Vec<DropSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DropSchema {
    pub code: String,
    pub quantity: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionCraftingRequest {
    pub code: String,
    pub quantity: i32,
}

impl QueryParameters for ActionCraftingRequest {
    fn get_path(parameter: String) -> String {
        ACTION_CRAFTING
            .path
            .to_string()
            .replace("{name}", &parameter)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionCraftingResponse {
    pub data: ActionCrafting,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionCrafting {
    pub cooldown: Cooldown,
    pub character: Character,
    pub details: SkillInfoSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionGeSellItemRequest {
    pub code: String,
    pub quantity: i32,
    pub price: f32,
}

impl QueryParameters for ActionGeSellItemRequest {
    fn get_path(parameter: String) -> String {
        ACTION_GE_SELL_ITEM
            .path
            .to_string()
            .replace("{name}", &parameter)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionGeSellItemResponse {
    pub data: GETransactionListSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GETransactionListSchema {
    pub cooldown: Cooldown,
    pub character: Character,
    pub transaction: GETransactionSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GETransactionSchema {
    pub item: String,
    pub quantity: i32,
    pub price: f32,
    pub total_price: f32,
}
