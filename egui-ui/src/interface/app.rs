use std::collections::HashMap;

use artifacts_mmo::api::{
    v4::{
        self,
        characters::Character,
        maps::Map,
        monsters::Monster,
        my_characters::GetMyCharactersRequest,
        status::{GameStatus, StatusRequest},
    },
    CharacterActionQueue, Endpoint, PageInput, PlayerAction,
};

use crate::constants;

pub const REPAINT_AFTER_SECONDS: f32 = 1_f32;

#[derive(Default, PartialEq)]
pub enum View {
    #[default]
    Play,
    Settings,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ApplicationState {
    pub account: Account,
    pub map_tiles: Vec<Map>,
    pub monsters: Vec<Monster>,

    #[serde(skip)]
    pub map_scene_view_bounds: egui::Rect,
    #[serde(skip)]
    pub view: View,
    #[serde(skip)]
    pub game_state: GameState,
    #[serde(skip)]
    pub system: System,
    #[serde(skip)]
    pub queue_action: QueueAction,
    #[serde(skip)]
    pub action_queues: HashMap<String, CharacterActionQueue>,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            account: Default::default(),
            game_state: Default::default(),
            monsters: Default::default(),
            map_tiles: Default::default(),
            system: Default::default(),
            queue_action: Default::default(),
            map_scene_view_bounds: egui::Rect::ZERO,
            view: Default::default(),
            action_queues: Default::default(),
        }
    }
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Account {
    pub api_token: String,
}

pub struct System {
    pub timezone: Option<chrono_tz::Tz>,
    pub http_client: ureq::Agent,
    pub promises: Promises,
    pub current_time: chrono::DateTime<chrono::Local>,
}

impl Default for System {
    fn default() -> Self {
        Self {
            timezone: None,
            http_client: ureq::agent(),
            promises: Default::default(),
            current_time: chrono::Local::now(),
        }
    }
}

#[derive(Default)]
pub struct Promises {
    pub system_status: Option<poll_promise::Promise<Option<GameStatus>>>,
    pub my_characters: Option<poll_promise::Promise<Vec<Character>>>,
    pub map_tiles: Option<poll_promise::Promise<Vec<Map>>>,
    pub movement: Option<poll_promise::Promise<Option<v4::my_characters::CharacterMovement>>>,
}

#[derive(Default)]
pub struct QueueAction {
    pub character_name: Option<String>,
    pub selected_action: Option<Action>,
    pub move_x: i32,
    pub move_y: i32,
}

#[derive(Default)]
pub struct GameState {
    pub server_status: Option<GameStatus>,
    pub characters: HashMap<String, Character>,
}

impl ApplicationState {
    /// Called once before the first frame.
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        match creation_context.egui_ctx.style().visuals.dark_mode {
            true => Self::configure_dark_mode(&creation_context.egui_ctx),
            false => Self::configure_light_mode(&creation_context.egui_ctx),
        }

        let mut state: ApplicationState = ApplicationState::default();
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = creation_context.storage {
            state = eframe::get_value(storage, constants::EGUI_STORAGE_APP_KEY).unwrap_or_default();
            return state;
        }

        // set_timezone(&mut state);
        state
    }

    fn configure_dark_mode(_context: &egui::Context) {}

    fn configure_light_mode(_context: &egui::Context) {}
}

impl eframe::App for ApplicationState {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, constants::EGUI_STORAGE_APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // NOTE: no File->Quit on web pages!
                    let is_web: bool = cfg!(target_arch = "wasm32");
                    if !is_web {
                        ui.menu_button("File", |ui| {
                            if ui.button("Quit").clicked() {
                                context.send_viewport_cmd(egui::ViewportCommand::Close);
                            };
                        });
                    }
                    egui::widgets::global_theme_preference_buttons(ui);
                    server_status_refresh_button(self, ui);
                    sync_character_button(self, ui);
                    sync_map_button(self, ui);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.horizontal_wrapped(|ui| {
                            egui::warn_if_debug_build(ui);
                            if let Some(timezone) = self.system.timezone {
                                ui.separator();
                                ui.label(timezone.to_string());
                            } else {
                                set_timezone(self);
                            }
                            server_status_widget(self, ui);
                            // ui.add(ClockWidget {
                            //     current_time: &self.device.current_time,
                            // });
                        });
                    });
                });
            });
        });
        egui::SidePanel::left("left_panel").show(context, |ui| {
            ui.vertical(|ui| {
                ui.selectable_value(&mut self.view, View::Play, "Play");
                ui.selectable_value(&mut self.view, View::Settings, "Settings");
            });
        });
        egui::SidePanel::right("right_panel")
            // .min_width(0.0f32)
            .show(context, |ui| {
                // task_widget(ui, self, "side_panel_tasks".to_string());
            });
        egui::CentralPanel::default().show(context, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            match self.view {
                View::Play => {
                    play_view_widget(self, ui);
                }
                View::Settings => {
                    ui.heading(constants::APPLICATION_TITLE);
                    api_token_widget(self, ui);
                    character_widget(self, ui);
                    play_widget(self, ui);
                }
            }

            // if self.device.widgets.settings.save_modal_open {
            //     let _ = &Modal::new(Id::new("save_result_modal")).show(ui.ctx(), |ui| {
            //         ui.set_width(150.0_f32);
            //         ui.heading(self.device.widgets.settings.save_result_message.as_str());
            //         ui.vertical_centered_justified(|ui| {
            //             if ui.button("Ok").clicked() {
            //                 self.device.widgets.settings.save_modal_open = false;
            //             }
            //         });
            //     });
            // }
            // search_widget(context, self);
            // if ui.input(|input_state| input_state.pointer.secondary_clicked()) {
            //     if self.device.search.search_open {
            //         return;
            //     }
            //     self.device.global_context_menu.is_open = true;
            //     self.device.global_context_menu.position =
            //         ui.input(|input_state| input_state.pointer.interact_pos());
            // }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |_| {});
        });

        if let Some(promise) = &self.system.promises.movement {
            if let Some(possible_movement) = promise.ready() {
                if let Some(movement) = possible_movement {
                    //TODO
                    log::info!("{:?}", movement.cooldown);
                    if let Some(character) = self
                        .game_state
                        .characters
                        .get_mut(&movement.character.profile.name)
                    {
                        *character = movement.character.clone();
                    }
                }
                log::info!("SETTING MOVE PROMISE TO NONE");
                self.system.promises.movement = None;
            }
        }
        if let Some(promise) = &self.system.promises.system_status {
            if let Some(server_status) = promise.ready() {
                self.game_state.server_status = server_status.to_owned();
                self.system.promises.system_status = None;
            }
        }
        if let Some(promise) = &self.system.promises.my_characters {
            if let Some(characters) = promise.ready() {
                self.game_state.characters = characters
                    .to_owned()
                    .into_iter()
                    .map(|character| (character.profile.name.clone(), character))
                    .collect();
                self.system.promises.my_characters = None;
            }
        }
        if let Some(promise) = &self.system.promises.map_tiles {
            if let Some(map_tiles) = promise.ready() {
                self.map_tiles = map_tiles.to_owned();
                self.system.promises.map_tiles = None;
            }
        }
        context.request_repaint_after_secs(REPAINT_AFTER_SECONDS);
        self.system.current_time = chrono::Local::now();
    }
}

pub fn configure_eframe_native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([
                constants::DEFAULT_INNER_SIZE_WIDTH,
                constants::DEFAULT_INNER_SIZE_HEIGHT,
            ])
            .with_min_inner_size([
                constants::DEFAULT_MINIMUM_INNER_SIZE_WIDTH,
                constants::DEFAULT_MINIMUM_INNER_SIZE_HEIGHT,
            ]), // .with_icon(
        //     // NOTE: Adding an icon is optional
        //     eframe::icon_data::from_png_bytes(&get_file_bytes(constants::APP_ICON_PATH))
        //         .unwrap_or(egui::IconData::default()),
        // )
        ..Default::default()
    }
}

pub fn set_timezone(state: &mut ApplicationState) {
    log::debug!("Attempting to set timezone.");
    match iana_time_zone::get_timezone() {
        Ok(timezone_string) => {
            match timezone_string.parse::<chrono_tz::Tz>() {
                Ok(timezone) => state.system.timezone = Some(timezone),
                Err(error) => log::error!("Failed to get timezone: {}", error),
            };
        }
        Err(error) => log::error!("Failed to get timezone: {}", error),
    }
}

pub fn server_status_widget(state: &mut ApplicationState, ui: &mut egui::Ui) {
    if let Some(status) = &state.game_state.server_status {
        ui.horizontal(|ui| {
            ui.label(format!("v{}", status.version));
            ui.label(status.status.clone());
            ui.label(status.characters_online.to_string());
        });
    }
}

pub fn server_status_refresh_button(state: &mut ApplicationState, ui: &mut egui::Ui) {
    if ui.button("Sync Server Status").clicked() {
        server_status_request(state);
    }
}

pub fn server_status_request(state: &mut ApplicationState) {
    let api_token = state.account.api_token.clone();
    let mut agent = state.system.http_client.clone();
    let promise = poll_promise::Promise::spawn_thread("server_status_thread", move || {
        let request: StatusRequest = StatusRequest::default();
        let result = match request.call(&mut agent, api_token) {
            artifacts_mmo::api::EndpointResponse::Error => {
                log::error!("Request ERROR!");
                None
            }
            artifacts_mmo::api::EndpointResponse::Success(response) => {
                return Some(response.data);
            }
        };
        result
    });
    state.system.promises.system_status = Some(promise);
}

pub fn character_image_widget(ui: &mut egui::Ui, uri: String) {
    ui.add(
        egui::Image::new(uri)
            .fit_to_original_size(0.5f32)
            .max_width(constants::CHARACTER_IMAGE_WIDTH)
            .max_height(constants::CHARACTER_IMAGE_HEIGHT),
    );
}

pub fn test_widget(ui: &mut egui::Ui) {
    ui.label("TESTING");
}

pub fn map_image_widget(ui: &mut egui::Ui, uri: String, map_tile_details: &Map) {
    let map_tile_text = format!(
        "{} ({}, {})",
        map_tile_details.name, map_tile_details.x, map_tile_details.y
    );
    let response = ui.add(
        egui::Image::new(uri)
            .fit_to_original_size(0.5f32)
            .max_width(constants::MAP_IMAGE_HEIGHT_WIDTH.1)
            .max_height(constants::MAP_IMAGE_HEIGHT_WIDTH.0),
    );

    if response.contains_pointer() {
        ui.painter()
            .rect_filled(response.rect, 0f32, egui::Color32::from_black_alpha(192));

        // ui.put(response.rect, egui::Label::new(&map_tile_text));
        // let layer_id = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("tile_overlay"));
        // ui.put(
        //     egui::Rect {
        //         min: response.rect.min,
        //         max: response.rect.min,
        //     },
        //     egui::Label::new(egui::RichText::new(&map_tile_text).size(10f32)),
        // );

        ui.painter().text(
            response.rect.center(),
            egui::Align2::CENTER_CENTER,
            &map_tile_text,
            egui::TextStyle::Heading.resolve(ui.style()),
            egui::Color32::WHITE,
        );
    };
}

pub fn sync_character_button(state: &mut ApplicationState, ui: &mut egui::Ui) {
    if ui.button("Sync Characters").clicked() {
        let api_token = state.account.api_token.clone();
        let mut agent = state.system.http_client.clone();
        let promise = poll_promise::Promise::spawn_thread("my_characters_thread", move || {
            let request: GetMyCharactersRequest = GetMyCharactersRequest::default();
            match request.call(&mut agent, api_token) {
                artifacts_mmo::api::EndpointResponse::Error => {
                    log::error!("Request ERROR!")
                }
                artifacts_mmo::api::EndpointResponse::Success(response) => {
                    return response.data;
                }
            };
            vec![]
        });
        state.system.promises.my_characters = Some(promise);
    }
}

pub fn sync_map_button(state: &mut ApplicationState, ui: &mut egui::Ui) {
    if ui.button("Sync Map").clicked() {
        let api_token = state.account.api_token.clone();
        let mut agent = state.system.http_client.clone();
        let promise = poll_promise::Promise::spawn_thread("map_sync_thread", move || {
            get_map_tiles(&api_token, &mut agent)
        });
        state.system.promises.map_tiles = Some(promise);
    }
}

pub fn get_map_tiles(api_key: &str, client: &mut ureq::Agent) -> Vec<Map> {
    let mut map_tiles = Vec::new();
    let first_page = 1u32;
    let mut request = v4::maps::GetAllMapsRequest {
        content_code: None,
        content_type: None,
        page_input: PageInput {
            size: Some(artifacts_mmo::api::PAGE_SIZE_MAX),
            page: Some(first_page),
        },
    };
    match request.call(client, api_key.to_string()) {
        artifacts_mmo::api::EndpointResponse::Error => log::error!("Failed to get map tiles."),
        artifacts_mmo::api::EndpointResponse::Success(mut response) => {
            map_tiles.append(&mut response.data);
            if response.page_output.pages.eq(&first_page) {
                return map_tiles;
            }

            for page in (first_page + 1)..response.page_output.pages.saturating_add(1) {
                request.page_input.page = Some(page);
                match request.call(client, api_key.to_string()) {
                    artifacts_mmo::api::EndpointResponse::Error => {
                        log::error!("Failed to get map tiles.Page: {}", page)
                    }
                    artifacts_mmo::api::EndpointResponse::Success(mut response) => {
                        map_tiles.append(&mut response.data)
                    }
                }
            }
        }
    };

    map_tiles
}

pub fn character_widget(state: &mut ApplicationState, ui: &mut egui::Ui) {
    let characters: Vec<Character> = state.game_state.characters.clone().into_values().collect();
    egui::ScrollArea::vertical()
        .auto_shrink(true)
        .show(ui, |ui| {
            egui::Grid::new("character_grid")
                .spacing(egui::vec2(ui.spacing().item_spacing.x * 3f32, 0.0))
                .show(ui, |ui| {
                    for character in characters.iter() {
                        character_image_widget(
                            ui,
                            v4::resources::ImageResourceType::Characters
                                .to_uri_string(&character.profile.skin.to_string()),
                        );
                        ui.label(character.profile.name.clone());
                        ui.label(format!(
                            "Location: ({}, {})",
                            character.location.x, character.location.y
                        ));
                        ui.label(format!(
                            "Cooldown expires: {}",
                            character.cooldown.cooldown_expiration
                        ));
                        add_action_to_queue_widget(state, ui, character.clone());
                        ui.end_row();
                    }
                });
        });
}

pub fn api_token_widget(state: &mut ApplicationState, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("API Token");
        ui.text_edit_singleline(&mut state.account.api_token);
    });
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Action {
    #[default]
    Move,
}

impl Action {
    pub fn iterator() -> impl Iterator<Item = Action> {
        [Action::Move].iter().copied()
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::Move => "Move",
            }
        )
    }
}

pub fn reset_queue_action(state: &mut ApplicationState) {
    state.queue_action.selected_action = None;
    state.queue_action.move_x = 0;
    state.queue_action.move_y = 0;
    state.queue_action.character_name = None;
}

pub fn add_action_to_queue_widget(
    state: &mut ApplicationState,
    ui: &mut egui::Ui,
    character: Character,
) {
    ui.horizontal(|ui| {
        if ui.button("Add action").clicked() {
            reset_queue_action(state);
            state.queue_action.character_name = Some(character.profile.name.clone());
        }
        if state
            .queue_action
            .character_name
            .clone()
            .is_some_and(|x| x.eq_ignore_ascii_case(&character.profile.name))
        {
            egui::ComboBox::from_id_salt(format!("new_action_{}", character.profile.name))
                .selected_text(match &state.queue_action.selected_action {
                    Some(action) => action.to_string(),
                    None => String::new(),
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.queue_action.selected_action, None, "None");
                    for action in Action::iterator() {
                        ui.selectable_value(
                            &mut state.queue_action.selected_action,
                            Some(action),
                            action.to_string(),
                        );
                    }
                });

            if let Some(action) = state.queue_action.selected_action {
                match action {
                    Action::Move => {
                        ui.horizontal(|ui| {
                            ui.label("Move");
                            ui.add(
                                egui::DragValue::new(&mut state.queue_action.move_x)
                                    .range(-5i32..=15i32)
                                    .prefix("x: "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut state.queue_action.move_y)
                                    .range(-5i32..=15i32)
                                    .prefix("y: "),
                            );
                        });
                    }
                }
            }
            if ui.button("Save").clicked() {
                let new_action: PlayerAction =
                    PlayerAction::Move(v4::my_characters::ActionMoveRequest {
                        name: character.profile.name.clone(),
                        body: v4::characters::Location {
                            x: state.queue_action.move_x,
                            y: state.queue_action.move_y,
                        },
                    });
                state
                    .action_queues
                    .entry(character.profile.name.clone())
                    .and_modify(|x| x.queue.push(new_action.clone()))
                    .or_insert(CharacterActionQueue {
                        character_name: character.profile.name.clone(),
                        paused: true,
                        queue: vec![new_action],
                    });
                reset_queue_action(state);
            }
        }
    });
}

pub fn play_widget(state: &mut ApplicationState, ui: &mut egui::Ui) {
    match state.action_queues.is_empty() {
        true => {
            ui.label("No actions queued.");
        }
        false => {
            ui.label("Queue");
            if ui.button("Clear").clicked() {
                state.action_queues.clear();
            }
            for (character_name, queue) in state.action_queues.iter_mut() {
                ui.vertical(|ui| {
                    ui.label(format!("Queue for:  {}", character_name.clone()));
                    ui.label(format!("Paused: {}", queue.paused));
                    for (i, action) in queue.queue.iter().enumerate() {
                        ui.label(format!(
                            "{} {}",
                            i.saturating_add(1).to_string(),
                            action.to_string()
                        ));
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Pause/Unpause").clicked() {
                        queue.paused = !queue.paused;
                    }
                    if ui.button("Delete").clicked() {}
                });
            }
            for (name, character_queue) in state.action_queues.clone().iter() {
                if character_queue.paused {
                    continue;
                }
                if let Some(x) = state.game_state.characters.get(name) {
                    if x.cooldown.cooldown_expiration > chrono::Utc::now() {
                        log::info!(
                            "Cooldown active for {}. Expires at: {} ",
                            name,
                            x.cooldown.cooldown_expiration
                        );
                        continue;
                    }
                }
                for action in character_queue.queue.iter() {
                    match action {
                        PlayerAction::Move(action_move_request) => {
                            move_character(state, action_move_request.clone())
                        }
                    }
                }
            }
        }
    }
}

pub fn move_character(state: &mut ApplicationState, request: v4::my_characters::ActionMoveRequest) {
    let api_token = state.account.api_token.clone();
    let mut agent = state.system.http_client.clone();
    let promise = poll_promise::Promise::spawn_thread("move_character_thread", move || {
        match request.call(&mut agent, api_token) {
            artifacts_mmo::api::EndpointResponse::Error => {
                log::error!("Request ERROR!")
            }
            artifacts_mmo::api::EndpointResponse::Success(response) => {
                return Some(response.data);
            }
        };

        None
    });

    state.system.promises.movement = Some(promise);
}

pub fn play_view_widget(state: &mut ApplicationState, ui: &mut egui::Ui) {
    egui::Frame::group(ui.style())
        .inner_margin(0.0f32)
        .show(ui, |ui| {
            let scene = egui::Scene::new().zoom_range(0.1..=2.0);
            let response = scene
                .show(ui, &mut state.map_scene_view_bounds, |ui| {
                    match state.map_tiles.is_empty() {
                        true => {
                            ui.label("Map not loaded.");
                        }
                        false => {
                            egui::Grid::new("map_grid")
                                .spacing(egui::vec2(ui.spacing().item_spacing.x * 0f32, 0.0))
                                .show(ui, |ui| {
                                    for (i, tile) in state.map_tiles.iter().enumerate() {
                                        map_image_widget(
                                            ui,
                                            v4::resources::ImageResourceType::Maps
                                                .to_uri_string(&tile.skin.to_string()),
                                            &tile,
                                        );
                                        if (i + 1) % 17 == 0 {
                                            ui.end_row();
                                        }
                                    }
                                });
                        }
                    }
                })
                .response;

            if response.double_clicked() {
                state.map_scene_view_bounds = egui::Rect::ZERO;
            }
        });
}
