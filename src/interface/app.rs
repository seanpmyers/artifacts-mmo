use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read},
};

use chrono::DateTime;
use dioxus::{
    desktop::{tao::window::Window, use_window},
    prelude::*,
};

use kira::{
    manager::{AudioManager, AudioManagerSettings, DefaultBackend},
    sound::static_sound::StaticSoundData,
};
use tracing::debug;
#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use super::style::theme::Theme;
use crate::{
    api::v1::{my_characters::MyCharacters, status::ServerStatus},
    constants::{AUDIO_PATHS, BUTTON_CLICK_SOUND, HERO_SIMPLE_CELEBRATION_03_SOUND},
    interface::router::route::Route,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationState {
    pub artifacts_server_status: ServerStatus,
    pub characters: Vec<MyCharacters>,
    pub current_theme: Theme,
    pub date_time: DateTime<chrono::Local>,
    pub full_day_month_date: String,
    pub full_timestamp_shorthand: String,
    pub session_start: DateTime<chrono::Local>,
    pub sound_on: bool,
    pub timezone: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assets {
    pub sounds: HashMap<String, Vec<u8>>,
}

pub static APPLICATION_STATE: GlobalSignal<ApplicationState> =
    Signal::global(|| ApplicationState {
        current_theme: Theme::Dark,
        date_time: chrono::Local::now(),
        session_start: chrono::Local::now(),
        timezone: iana_time_zone::get_timezone().unwrap_or("Unknown Timezone".to_string()),
        full_day_month_date: "".to_string(),
        full_timestamp_shorthand: "".to_string(),
        artifacts_server_status: ServerStatus::Unknown,
        characters: Vec::new(),
        sound_on: true,
    });

pub static ASSETS: GlobalSignal<Assets> = Signal::global(|| Assets {
    sounds: HashMap::new(),
});

#[component]
pub fn App() -> Element {
    blur_window(&use_window().window);

    use_future(move || async move {
        load_sounds();
        play_hero_simple_celebration_03();
    });

    rsx! {
        Router::<Route> {}
    }
}

pub enum Action {
    PlaySound(Vec<u8>),
}

pub fn load_sounds() {
    for (path, key) in AUDIO_PATHS {
        let mut file: File = File::open(path).unwrap();
        let mut sound_bytes: Vec<u8> = Vec::new();
        file.read_to_end(&mut sound_bytes).unwrap();
        ASSETS.write().sounds.insert(key.to_string(), sound_bytes);
    }
    debug!("Sounds loaded.");
}

pub fn play_sound(sound_bytes: Vec<u8>) {
    let cursor: Cursor<Vec<u8>> = Cursor::new(sound_bytes);
    let mut manager: AudioManager =
        AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let sound_data: StaticSoundData = StaticSoundData::from_cursor(cursor).unwrap();
    manager.play(sound_data.clone()).unwrap();
}

pub fn play_button_click_sound() {
    if !APPLICATION_STATE().sound_on {
        return;
    }
    if let Some(sound_bytes) = ASSETS.read().sounds.get(BUTTON_CLICK_SOUND) {
        let sound_bytes: Vec<u8> = (*sound_bytes).clone();
        play_sound(sound_bytes);
    }
}

pub fn play_hero_simple_celebration_03() {
    if !APPLICATION_STATE().sound_on {
        return;
    }
    if let Some(sound_bytes) = ASSETS.read().sounds.get(HERO_SIMPLE_CELEBRATION_03_SOUND) {
        let sound_bytes: Vec<u8> = (*sound_bytes).clone();
        play_sound(sound_bytes);
    }
}

pub fn blur_window(window: &Window) {
    #[cfg(target_os = "windows")]
    apply_acrylic(window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    #[cfg(target_os = "macos")]
    apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
}
