use dioxus::prelude::*;

use crate::interface::{app::APPLICATION_STATE, icon::THEME_TOGGLE_SVG, style::theme::Theme};

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
                }
            },
            class: "icon ",
            dangerous_inner_html: THEME_TOGGLE_SVG
        }
    }
}
