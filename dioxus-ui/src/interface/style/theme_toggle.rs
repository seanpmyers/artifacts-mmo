use dioxus::prelude::*;

use crate::{
    constants::css::IMAGE_ICON,
    interface::{
        app::APPLICATION_STATE, style::theme::Theme, widget::audible_button::AudibleButton,
    },
};

pub const LIGHT: &str = "light";
pub const DEFAULT_TOGGLE_TITLE: &str = "Switch color theme";
pub const BUTTON_CLASS: &str = "theme-toggle";

#[component]
pub fn ThemeToggle() -> Element {
    rsx! {
        AudibleButton {
            onclick: move |_| {
                APPLICATION_STATE
                    .write()
                    .current_theme = match APPLICATION_STATE().current_theme {
                    Theme::Dark => Theme::Light,
                    Theme::Light => Theme::Dark,
                };
            },
            tooltip: match APPLICATION_STATE().current_theme {
                Theme::Dark => "Enable light mode".to_string(),
                Theme::Light => "Enable dark mode".to_string(),
            },
            img { class: IMAGE_ICON, src: asset!("/assets/images/theme_toggle.png") }
        }
    }
}
