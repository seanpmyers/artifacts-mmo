use artifacts_mmo::api::{
    v4::{
        self,
        characters::Character,
        maps::Map,
        monsters::Monster,
        my_characters::GetMyCharactersRequest,
        status::{GameStatus, StatusRequest},
    },
    Endpoint,
};

use crate::constants;

pub const REPAINT_AFTER_SECONDS: f32 = 1_f32;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ApplicationState {
    pub account: Account,
    pub server_status: Option<GameStatus>,
    pub characters: Vec<Character>,
    pub monsters: Vec<Monster>,
    pub map_tiles: Vec<Map>,
    #[serde(skip)]
    pub system: System,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            account: Default::default(),
            server_status: Default::default(),
            characters: Default::default(),
            monsters: Default::default(),
            map_tiles: Default::default(),
            system: Default::default(),
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
}

impl Default for System {
    fn default() -> Self {
        Self {
            timezone: None,
            http_client: ureq::agent(),
            promises: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct Promises {
    pub system_status: Option<poll_promise::Promise<Option<GameStatus>>>,
    pub my_characters: Option<poll_promise::Promise<Vec<Character>>>,
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

            // ui.vertical(|ui| {
            //     ui.selectable_value(&mut self.device.current_tab, Tab::Chronicle, "Chronicle");
            //     ui.selectable_value(&mut self.device.current_tab, Tab::Lore, "Lore");
            //     ui.selectable_value(&mut self.device.current_tab, Tab::You, "You");
            //     ui.selectable_value(&mut self.device.current_tab, Tab::Timeline, "Timeline");
            //     ui.selectable_value(&mut self.device.current_tab, Tab::Settings, "Settings");
            // });
        });
        egui::SidePanel::right("right_panel")
            .min_width(0.0f32)
            .show(context, |ui| {
                // task_widget(ui, self, "side_panel_tasks".to_string());
            });
        egui::CentralPanel::default().show(context, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(constants::APPLICATION_TITLE);
            api_token_widget(self, ui);
            character_widget(ui, &self.characters);
            // match self.device.current_tab {
            //     Tab::Chronicle => chronicle_widget(ui, self),
            //     Tab::Lore => lore_widget(ui, self),
            //     Tab::Word => word_view(ui, self),
            //     Tab::You => you(ui, self),
            //     Tab::Settings => settings(ui, self),
            //     Tab::Timeline => timeline_widget(ui, self),
            // }

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
        // if self.device.global_context_menu.is_open {
        //     if let Some(right_click_position) = self.device.global_context_menu.position {
        //         Window::new("Quick Actions")
        //             .id("global_context_menu".into())
        //             .fixed_pos(right_click_position)
        //             .collapsible(false)
        //             .resizable(false)
        //             .frame(egui::Frame::popup(&context.style()))
        //             .show(context, |ui| {
        //                 save_load_buttons(ui, self);
        //             });
        //     }
        // }
        if let Some(promise) = &self.system.promises.system_status {
            if let Some(server_status) = promise.ready() {
                self.server_status = server_status.to_owned();
            }
        }
        if let Some(promise) = &self.system.promises.my_characters {
            if let Some(characters) = promise.ready() {
                self.characters = characters.to_owned();
            }
        }
        context.request_repaint_after_secs(REPAINT_AFTER_SECONDS);
        // self.device.current_time = chrono::Local::now();
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
    if let Some(status) = &state.server_status {
        ui.horizontal(|ui| {
            ui.label(format!("v{}", status.version));
            ui.label(status.status.clone());
            ui.label(status.characters_online.to_string());
        });
    }
}

pub fn server_status_refresh_button(state: &mut ApplicationState, ui: &mut egui::Ui) {
    if ui.button("Sync Server Status").clicked() {
        let api_token = state.account.api_token.clone();
        let mut agent = state.system.http_client.clone();
        let context = ui.ctx().clone();
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
            context.request_repaint();
            result
        });
        state.system.promises.system_status = Some(promise);
    }
}

pub fn character_image_widget(ui: &mut egui::Ui, uri: String) {
    ui.add(
        egui::Image::new(uri)
            .fit_to_original_size(0.5f32)
            .max_width(constants::CHARACTER_IMAGE_WIDTH)
            .max_height(constants::CHARACTER_IMAGE_HEIGHT),
    );
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

pub fn character_widget(ui: &mut egui::Ui, characters: &Vec<Character>) {
    egui::ScrollArea::vertical()
        .auto_shrink(false)
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
