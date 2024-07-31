use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{ApiEndpoint, HttpRequestMethod},
};

pub const GET_ALL_CHARACTERS: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/characters",
    http_request_method: HttpRequestMethod::GET,
};
pub const GET_CHARACTER: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/characters/{name}",
    http_request_method: HttpRequestMethod::GET,
};

pub const CREATE_CHARACTER: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/characters/create",
    http_request_method: HttpRequestMethod::POST,
};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Skin {
    Men1,
    Men2,
    Men3,
    Women1,
    Women2,
    Women3,
}

pub struct GetCharactersResponse {
    pub data: Vec<Character>,
    pub total: i32,
    pub page: i32,
    pub size: i32,
    pub pages: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharacterResponse {
    pub data: Character,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
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
    pub inventory: Vec<InventorySlot>,
    pub inventory_max_items: i32,
    pub task: String,
    pub task_type: String,
    pub task_progress: i32,
    pub task_total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventorySlot {
    pub slot: i32,
    pub code: String,
    pub quantity: i32,
}

impl Character {
    pub fn current_inventory_quantity_total(&self) -> i32 {
        self.inventory.iter().map(|slot| slot.quantity).sum()
    }

    pub fn is_inventory_full(&self) -> bool {
        self.current_inventory_quantity_total() >= self.inventory_max_items
    }

    pub fn wait_for_cooldown(&self) {
        if self.cooldown > 0 {
            info!(
                "Waiting {} seconds for cooldown to expire for character {}...",
                self.cooldown, self.name
            );
            std::thread::sleep(std::time::Duration::from_secs(self.cooldown as u64));
        }
    }
}
