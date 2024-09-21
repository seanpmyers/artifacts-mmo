use dioxus::prelude::*;

use crate::{
    api::v1::status::ServerStatus,
    interface::{
        app::APPLICATION_STATE, icon::HOME_SVG, router::route::Route,
        style::theme_toggle::ThemeToggle,
    },
};

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: format!(
                "{} container",
                APPLICATION_STATE().current_theme.to_string().to_lowercase(),
            ),
            div { class: "top-nav",
                h3 { class: "top-nav-heading", "Artifacts-MMO" }
                span { class: format!("online-status {}", match APPLICATION_STATE().artifacts_server_status {
                    ServerStatus::Online => "green",
                    ServerStatus::Offline => "red",
                    ServerStatus::Unknown => "gray"
                })}
                ThemeToggle {}
            }
            div { class: "side-nav",
                div { class: "side-nav-container",
                    Link { class: "button side-nav-item", to: Route::Home {},
                        div { class: "icon", dangerous_inner_html: HOME_SVG }
                        span { "Home" }
                    }
                }
            }
            main { class: "", Outlet::<Route> {} }
        }
    }
}
