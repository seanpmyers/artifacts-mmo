use dioxus::prelude::*;

use crate::{
    api::v1::status::ServerStatus,
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
					        ServerStatus::Online => "green",
					        ServerStatus::Offline => "red",
					        ServerStatus::Unknown => "gray",
					    },
					)
				}
			}
		}
}
