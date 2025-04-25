use artifacts_mmo::api::{
    v4::{self},
    Endpoint, EndpointResponse,
};
use dioxus::prelude::*;

use crate::{
    constants::css::{self},
    interface::{app::APPLICATION_STATE, widget::audible_button::AudibleButton},
};

#[component]
pub fn Status() -> Element {
    let next_server_status_refresh: Signal<String> = use_signal(|| "".to_string());
    let server_status: Memo<String> = use_memo(move || {
        if let Some(status) = APPLICATION_STATE().sever_status.data {
            format!(
                "Version: {}\nStatus: {}\nCharacters Online: {}\nLast Wipe: {}\nNext Wipe: {}\nAnnouncements: {}",
                status.version,
                status.status,
                status.characters_online,
                status.last_wipe,
                status.next_wipe,
                serde_json::to_string_pretty(&status.announcements).unwrap_or("None".to_string())
            )
        } else {
            "Offline".to_string()
        }
    });

    rsx! {
        div { class: css::CANVAS,
            div { class: css::CANVAS,
                div {
                    label { "Server Status" }
                    AudibleButton { onclick: |_| refresh_status(), tooltip: "Refresh status".to_string(),
                        img {
                            class: css::IMAGE_ICON,
                            src: asset!("assets/images/refresh.png")
                        }
                    }
                    label { "Next refresh in: {next_server_status_refresh}s" }
                }
                pre {
                    code { class: css::CODE, language: "json", "{server_status}" }
                }
            }
        }
    }
}

pub fn refresh_status() {
    let mut http_client: ureq::Agent = ureq::agent();
    let mut request = v4::status::StatusRequest {};
    let response: EndpointResponse<v4::status::StatusResponse> = v4::status::StatusRequest::call(
        &mut request,
        &mut http_client,
        APPLICATION_STATE.read().api_token.clone(),
    );
    match response {
        EndpointResponse::Error => log::error!("Status request failed."),
        EndpointResponse::Success(status) => {
            if status.data.status == "online" {
                APPLICATION_STATE.write().artifacts_server_status =
                    v4::status::ServerStatus::Online;
            } else {
                APPLICATION_STATE.write().artifacts_server_status =
                    v4::status::ServerStatus::Offline;
            }
            APPLICATION_STATE.write().sever_status.sync_now(status.data);
        }
    }
}

pub async fn check_server_status(mut next_server_status_refresh: Signal<String>) {
    let mut last_updated: tokio::time::Instant = tokio::time::Instant::now();
    let mut time_to_update: bool = true;
    loop {
        match time_to_update {
            true => {
                refresh_status();
                last_updated = tokio::time::Instant::now();
                time_to_update = false;
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            false => match (tokio::time::Instant::now() - last_updated)
                >= std::time::Duration::from_secs(60 * 60)
            {
                true => time_to_update = true,
                false => {
                    next_server_status_refresh.set(
                        (std::time::Duration::from_secs(60 * 60)
                            - (tokio::time::Instant::now() - last_updated))
                            .as_secs()
                            .to_string(),
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            },
        }
    }
}
