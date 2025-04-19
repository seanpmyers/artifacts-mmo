pub mod accounts {
    use crate::api::Endpoint;

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct CreateAccountRequest {
        pub email: Option<String>,
        pub password: String,
        pub username: String,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct CreateAccountResponse {}

    impl Endpoint for CreateAccountRequest {
        fn http_request_method() -> http::Method {
            http::Method::POST
        }

        fn pageable() -> bool {
            false
        }

        fn path(&self) -> String {
            "/accounts/create".to_string()
        }

        fn query(&self) -> Option<String> {
            None
        }

        type Response = CreateAccountResponse;

        fn request_body(&self) -> Option<CreateAccountRequest> {
            Some(self.clone())
        }
    }
}

pub mod my_account {}
pub mod my_characters {}
pub mod status {
    use crate::api::Endpoint;

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct StatusRequest {}

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct StatusResponse {
        data: GameStatus,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct GameStatus {
        pub announcements: Option<Vec<Announcements>>,
        pub characters_online: i32,
        pub last_wipe: String,
        pub max_level: i32,
        pub next_wipe: String,
        #[serde(with = "chrono::serde::ts_seconds_option")]
        pub server_time: Option<chrono::DateTime<chrono::Utc>>,
        pub status: String,
        pub version: String,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct Announcements {
        pub message: String,
        #[serde(with = "chrono::serde::ts_seconds_option")]
        pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub enum ServerStatus {
        Offline,
        Online,
        #[default]
        Unknown,
    }

    impl std::fmt::Display for ServerStatus {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ServerStatus::Online => write!(formatter, "Online"),
                ServerStatus::Unknown => write!(formatter, "Unknown"),
                ServerStatus::Offline => write!(formatter, "Offline"),
            }
        }
    }

    impl Endpoint for StatusRequest {
        type Response = StatusResponse;

        fn http_request_method() -> http::Method {
            http::Method::GET
        }

        fn pageable() -> bool {
            false
        }

        fn path(&self) -> String {
            "/".to_string()
        }

        fn query(&self) -> Option<String> {
            None
        }

        fn request_body(&self) -> Option<Self>
        where
            Self: Sized,
        {
            None
        }
    }
}
pub mod achievements {}
pub mod badges {}
pub mod characters {
    use crate::api::Endpoint;

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct GetCharacterRequest {
        pub name: String,
    }

    impl Endpoint for GetCharacterRequest {
        type Response = GetCharacterResponse;

        fn http_request_method() -> http::Method {
            http::Method::GET
        }

        fn pageable() -> bool {
            false
        }

        fn path(&self) -> String {
            format!("/characters/{}", self.name)
        }

        fn query(&self) -> Option<String> {
            None
        }

        fn request_body(&self) -> Option<Self>
        where
            Self: Sized,
        {
            None
        }
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct GetCharactersResponse {
        pub data: Vec<Character>,
        pub total: i32,
        pub page: i32,
        pub size: i32,
        pub pages: i32,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    pub struct GetCharacterResponse {
        pub data: Character,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
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
        pub cooldown: Cooldown,
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
    pub struct Cooldown {
        pub cooldown: i32,
        pub cooldown_expiration: chrono::DateTime<chrono::Utc>,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct Stats {
        pub haste: i32,
        pub prospecting: i32,
        pub speed: i32,
        pub stamina: i32,
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
}
pub mod effects {}
pub mod events {}
pub mod grand_exchange {}
pub mod items {}
pub mod leaderboard {}
pub mod maps {
    use crate::api::Endpoint;

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct GetMapRequest {
        x: i32,
        y: i32,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct GetMapResponse {
        data: Map,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct GetAllMapsRequest {
        pub content_code: Option<String>,
        pub content_type: Option<MapContentType>,
        pub page_size: Option<i32>,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct GetAllMapsResponse {
        pub data: Vec<Map>,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct Map {
        pub content: Option<MapContent>,
        pub name: String,
        pub skin: String,
        pub x: i32,
        pub y: i32,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub struct MapContent {
        pub code: String,
        #[serde(rename = "type")]
        pub content_type: MapContentType,
    }

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum MapContentType {
        #[default]
        Bank,
        GrandExchange,
        Monster,
        Npc,
        Resource,
        TasksMaster,
        Workshop,
    }

    impl std::fmt::Display for MapContentType {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                formatter,
                "{}",
                match self {
                    MapContentType::Bank => "bank",
                    MapContentType::GrandExchange => "grand_exchange",
                    MapContentType::Monster => "monster",
                    MapContentType::Npc => "npc",
                    MapContentType::Resource => "resource",
                    MapContentType::TasksMaster => "tasks_master",
                    MapContentType::Workshop => "workshop",
                }
            )
        }
    }

    impl Endpoint for GetMapRequest {
        type Response = GetMapResponse;

        fn http_request_method() -> http::Method {
            http::Method::GET
        }

        fn pageable() -> bool {
            false
        }

        fn path(&self) -> String {
            format!("/maps/{}/{}", self.x, self.y)
        }

        fn query(&self) -> Option<String> {
            None
        }

        fn request_body(&self) -> Option<Self>
        where
            Self: Sized,
        {
            None
        }
    }

    impl Endpoint for GetAllMapsRequest {
        type Response = GetAllMapsResponse;

        fn http_request_method() -> http::Method {
            http::Method::GET
        }

        fn pageable() -> bool {
            true
        }

        fn path(&self) -> String {
            "/maps".to_string()
        }

        fn query(&self) -> Option<String> {
            let page_query: String = format!(
                "&size={}",
                match self.page_size {
                    Some(size) => size,
                    None => Self::default_page_size(),
                }
            );
            match (&self.content_code, &self.content_type) {
                (None, None) => None,
                (None, Some(content_type)) => Some(format!(
                    "content_type={}{}",
                    content_type.to_string(),
                    page_query
                )),
                (Some(content_code), None) => {
                    Some(format!("content_code={}{}", content_code, page_query))
                }
                (Some(content_code), Some(content_type)) => Some(format!(
                    "content_code={}&content_type={}{}",
                    content_code, content_type, page_query
                )),
            }
        }

        fn request_body(&self) -> Option<Self>
        where
            Self: Sized,
        {
            None
        }
    }
}
pub mod monsters {}
pub mod npcs {}
pub mod resources {
    use crate::api::ARTIFACTS_MMO_HOST;

    #[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
    pub enum ImageResourceType {
        #[default]
        Characters,
        Effects,
        Items,
        Maps,
        Monsters,
        Resources,
    }

    impl std::fmt::Display for ImageResourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::Characters => "characters",
                    Self::Items => "items",
                    Self::Monsters => "monsters",
                    Self::Maps => "maps",
                    Self::Resources => "resources",
                    Self::Effects => "effects",
                }
            )
        }
    }

    impl ImageResourceType {
        pub fn to_uri_string(&self, code: &str) -> String {
            format!("{}{}.png", ARTIFACTS_MMO_HOST, Self::path(&self, code))
        }

        pub fn path(&self, code: &str) -> String {
            format!("/images/{}/{}.png", self.to_string(), code)
        }

        pub fn to_uri(&self, code: &str) -> Result<http::Uri, http::Error> {
            http::Uri::builder()
                .scheme(http::uri::Scheme::HTTPS)
                .authority(ARTIFACTS_MMO_HOST)
                .path_and_query(&Self::path(self, code))
                .build()
        }
    }
}
pub mod tasks {}
pub mod token {}
