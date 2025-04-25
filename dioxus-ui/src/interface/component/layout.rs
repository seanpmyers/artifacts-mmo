use dioxus::prelude::*;

use crate::interface::{
    app::APPLICATION_STATE,
    component::navigation::SideNavigation,
    router::route::Route,
    style::theme_toggle::ThemeToggle,
    widget::{server_status::ServerStatus, sound::Sound},
};

#[component]
pub fn Layout() -> Element {
    let version = use_memo(move || match APPLICATION_STATE().sever_status.data {
        Some(data) => data.version,
        None => "".to_string(),
    });

    rsx! {
        div {
            class: format!(
                "{} container",
                APPLICATION_STATE().current_theme.to_string().to_lowercase(),
            ),
            div { class: "top-nav",
                h3 { class: "top-nav-heading", "Artifacts-MMO" }
                if !version().is_empty() {
                    div { "v{version}" }
                }
                ServerStatus {}
                Sound {}
                ThemeToggle {}
            }
            SideNavigation {}
            main { class: "", Outlet::<Route> {} }
        }
    }
}
