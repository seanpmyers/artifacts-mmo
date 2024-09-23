use dioxus::prelude::*;

use crate::{api::v1::status::ServerStatus, interface::app::APPLICATION_STATE};

pub const CANVAS_CLASS: &str = "canvas";
pub const SERVER_STATUS_CLASS: &str = "server-status";

#[component]
pub fn ServerStatus() -> Element {
    rsx! {
			div { class: format!("{} {}", CANVAS_CLASS, SERVER_STATUS_CLASS),
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
