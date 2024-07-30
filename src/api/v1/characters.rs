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
}

impl Character {
    pub fn current_inventory_quantity_total(&self) -> i32 {
        let mut count: i32 = 0;
        count += self.inventory_slot1_quantity;
        count += self.inventory_slot2_quantity;
        count += self.inventory_slot3_quantity;
        count += self.inventory_slot4_quantity;
        count += self.inventory_slot5_quantity;
        count += self.inventory_slot6_quantity;
        count += self.inventory_slot7_quantity;
        count += self.inventory_slot8_quantity;
        count += self.inventory_slot9_quantity;
        count += self.inventory_slot10_quantity;
        count += self.inventory_slot11_quantity;
        count += self.inventory_slot12_quantity;
        count += self.inventory_slot13_quantity;
        count += self.inventory_slot14_quantity;
        count += self.inventory_slot15_quantity;
        count += self.inventory_slot16_quantity;
        count += self.inventory_slot17_quantity;
        count += self.inventory_slot18_quantity;
        count += self.inventory_slot19_quantity;
        count += self.inventory_slot20_quantity;
        count
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
