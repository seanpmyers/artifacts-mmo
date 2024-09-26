use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use chrono::DateTime;
use dioxus::{desktop::use_window, prelude::*};

use rodio::Source;
#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use super::style::theme::Theme;
use crate::{
    api::v1::{my_characters::MyCharacters, status::ServerStatus},
    constants::BUTTON_CLICK_SOUND_PATH,
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
    #[cfg(target_os = "windows")]
    apply_acrylic(&use_window().window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    #[cfg(target_os = "macos")]
    apply_vibrancy(
        &use_window().window,
        NSVisualEffectMaterial::HudWindow,
        None,
        None,
    )
    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    use_future(|| async move {
        load_sounds();
    });

    rsx! {
        Router::<Route> {}
    }
}

pub fn load_sounds() {
    let mut file: File = File::open(BUTTON_CLICK_SOUND_PATH).unwrap();
    let mut sound_bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut sound_bytes).unwrap();
    ASSETS
        .write()
        .sounds
        .insert("button_click".to_string(), sound_bytes);
}

pub fn play_sound(sound_bytes: Vec<u8>) {
    let cursor = std::io::Cursor::new(sound_bytes);
    let sound = rodio::Decoder::new(cursor).unwrap();
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink: rodio::Sink = rodio::Sink::try_new(&handle).unwrap();
    sink.append(sound);
    sink.sleep_until_end();
}
