use chrono::DateTime;
use dioxus::{desktop::use_window, prelude::*};

#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use super::style::theme::Theme;
use crate::{
    api::v1::{my_characters::MyCharacters, status::ServerStatus},
    interface::router::route::Route,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationState {
    pub current_theme: Theme,
    pub date_time: DateTime<chrono::Local>,
    pub full_day_month_date: String,
    pub full_timestamp_shorthand: String,
    pub session_start: DateTime<chrono::Local>,
    pub timezone: String,
    pub artifacts_server_status: ServerStatus,
    pub characters: Vec<MyCharacters>,
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

    use_future(|| async move {});

    rsx! {
        Router::<Route> {}
    }
}
