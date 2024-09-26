use dioxus::prelude::*;

use crate::{
    constants::css::ICON,
    interface::{
        app::{play_sound, APPLICATION_STATE, ASSETS},
        icon::THEME_TOGGLE_SVG,
        style::theme::Theme,
    },
};

pub const LIGHT: &str = "light";
pub const DEFAULT_TOGGLE_TITLE: &str = "Switch color theme";
pub const BUTTON_CLASS: &str = "theme-toggle";

#[component]
pub fn ThemeToggle() -> Element {
    rsx! {
        button {
            onclick: move |_| {
                APPLICATION_STATE
                    .write()
                    .current_theme = match APPLICATION_STATE().current_theme {
                    Theme::Dark => Theme::Light,
                    Theme::Light => Theme::Dark,
                };
                if !APPLICATION_STATE().sound_on {
                    return;
                }
                if let Some(sound_bytes) = ASSETS.read().sounds.get("button_click") {
                    let sound_bytes = (*sound_bytes).clone();
                    std::thread::spawn(move || {
                        play_sound(sound_bytes);
                    });
                }
            },
            class: ICON,
            dangerous_inner_html: THEME_TOGGLE_SVG
        }
    }
}
