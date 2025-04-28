use crate::api::{CharacterActionQueue, Endpoint, NoBody, PageOutput};

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetCharacterRequest {
    pub name: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetCharactersResponse {
    pub data: Vec<Character>,
    #[serde(flatten)]
    pub page_ouptut: PageOutput,
}
impl Endpoint for GetCharacterRequest {
    type Response = GetCharacterResponse;
    type RequestBody = NoBody;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn path(&self) -> String {
        format!("/characters/{}", self.name)
    }

    fn get_default_host() -> &'static str {
        crate::api::ARTIFACTS_MMO_API_HOST
    }
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetCharacterResponse {
    pub data: Character,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Skin {
    #[default]
    Men1,
    Men2,
    Men3,
    Women1,
    Women2,
    Women3,
}

impl std::fmt::Display for Skin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Skin::Men1 => "men1",
                Skin::Men2 => "men2",
                Skin::Men3 => "men3",
                Skin::Women1 => "women1",
                Skin::Women2 => "women2",
                Skin::Women3 => "women3",
            }
        )
    }
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Character {
    #[serde(flatten)]
    pub alchemy: Alchemy,
    #[serde(flatten)]
    pub combat: Combat,
    #[serde(flatten)]
    pub cooking: Cooking,
    #[serde(flatten)]
    pub cooldown: CharacterCooldown,
    #[serde(flatten)]
    pub equipment: Equipment,
    #[serde(flatten)]
    pub fishing: Fishing,
    #[serde(flatten)]
    pub gear_crafting: GearCrafting,
    #[serde(flatten)]
    pub health: Health,
    #[serde(flatten)]
    pub inventory: Inventory,
    #[serde(flatten)]
    pub jewelry_crafting: JewelryCrafting,
    #[serde(flatten)]
    pub level: Level,
    #[serde(flatten)]
    pub location: Location,
    #[serde(flatten)]
    pub mining: Mining,
    #[serde(flatten)]
    pub profile: Profile,
    #[serde(flatten)]
    pub resistance: Resistance,
    #[serde(flatten)]
    pub stats: Stats,
    #[serde(flatten)]
    pub task: Task,
    #[serde(flatten)]
    pub weapon_crafting: WeaponCrafting,
    #[serde(flatten)]
    pub wood_cutting: WoodCutting,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Profile {
    pub account: String,
    pub name: String,
    pub skin: Skin,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Inventory {
    pub gold: i32,
    pub inventory: Vec<InventorySlot>,
    pub inventory_max_items: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct InventorySlot {
    pub code: String,
    pub quantity: i32,
    pub slot: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Equipment {
    pub amulet_slot: String,
    pub artifact1_slot: String,
    pub artifact2_slot: String,
    pub artifact3_slot: String,
    pub bag_slot: String,
    pub body_armor_slot: String,
    pub boots_slot: String,
    pub helmet_slot: String,
    pub leg_armor_slot: String,
    pub ring1_slot: String,
    pub ring2_slot: String,
    pub rune_slot: String,
    pub shield_slot: String,
    pub utility1_slot: String,
    pub utility1_slot_quantity: u32,
    pub utility2_slot: String,
    pub utility2_slot_quantity: u32,
    pub weapon_slot: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct CharacterCooldown {
    pub cooldown: i32,
    pub cooldown_expiration: chrono::DateTime<chrono::Utc>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Stats {
    pub haste: i32,
    pub prospecting: i32,
    pub speed: i32,
    pub stamina: Option<i32>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Health {
    pub max_hp: i32,
    pub hp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Level {
    pub level: i32,
    pub max_xp: i32,
    pub xp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Combat {
    pub attack_air: i32,
    pub attack_earth: i32,
    pub attack_fire: i32,
    pub attack_water: i32,
    pub critical_strike: i32,
    pub dmg: i32,
    pub dmg_air: f32,
    pub dmg_earth: f32,
    pub dmg_fire: f32,
    pub dmg_water: f32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Task {
    pub task: String,
    pub task_progress: i32,
    pub task_total: i32,
    pub task_type: String,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Alchemy {
    pub alchemy_level: i32,
    pub alchemy_max_xp: i32,
    pub alchemy_xp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Mining {
    pub mining_level: i32,
    pub mining_max_xp: i32,
    pub mining_xp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Resistance {
    pub res_air: f32,
    pub res_earth: f32,
    pub res_fire: f32,
    pub res_water: f32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Cooking {
    pub cooking_level: i32,
    pub cooking_max_xp: i32,
    pub cooking_xp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct JewelryCrafting {
    pub jewelrycrafting_level: i32,
    pub jewelrycrafting_max_xp: i32,
    pub jewelrycrafting_xp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct WoodCutting {
    pub woodcutting_level: i32,
    pub woodcutting_max_xp: i32,
    pub woodcutting_xp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct WeaponCrafting {
    pub weaponcrafting_level: i32,
    pub weaponcrafting_max_xp: i32,
    pub weaponcrafting_xp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GearCrafting {
    pub gearcrafting_level: i32,
    pub gearcrafting_max_xp: i32,
    pub gearcrafting_xp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Fishing {
    pub fishing_level: i32,
    pub fishing_max_xp: i32,
    pub fishing_xp: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Character {
    pub fn to_name_map(characters: &Vec<Self>) -> std::collections::HashMap<String, Character> {
        std::collections::HashMap::from_iter(
            characters
                .iter()
                .map(|character| (character.profile.name.clone(), character.clone())),
        )
    }

    pub fn to_index_map(characters: &Vec<Self>) -> std::collections::HashMap<String, usize> {
        std::collections::HashMap::from_iter(
            characters
                .iter()
                .enumerate()
                .map(|(i, character)| (character.profile.name.clone(), i)),
        )
    }

    pub fn to_action_queue_list(characters: &Vec<Self>) -> Vec<CharacterActionQueue> {
        characters
            .iter()
            .map(|character| CharacterActionQueue {
                character_name: character.profile.name.clone(),
                paused: true,
                queue: Vec::new(),
            })
            .collect()
    }
}
