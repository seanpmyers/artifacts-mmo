use dioxus::prelude::*;

use crate::{
    constants::css::{CANVAS, SERVER_STATUS},
    interface::app::APPLICATION_STATE,
};

#[component]
pub fn ServerStatus() -> Element {
    rsx! {
        div { class: format!("{} {}", CANVAS, SERVER_STATUS),
            span { "{APPLICATION_STATE().artifacts_server_status.to_string()}" }
            span {
                class: format!(
                    "online-status {}",
                    match APPLICATION_STATE().artifacts_server_status {
                        ServerStatusState::Online => "green",
                        ServerStatusState::Offline => "red",
                        ServerStatusState::Unknown => "gray",
                    },
                )
            }
        }
    }
}
