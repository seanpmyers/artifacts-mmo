use dioxus::prelude::*;

use crate::interface::{
    app::APPLICATION_STATE, component::navigation::SideNavigation, router::route::Route,
    style::theme_toggle::ThemeToggle, widget::server_status::ServerStatus,
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
                ServerStatus {}
                ThemeToggle {}
            }
            SideNavigation {}
            main { class: "", Outlet::<Route> {} }
        }
    }
}
