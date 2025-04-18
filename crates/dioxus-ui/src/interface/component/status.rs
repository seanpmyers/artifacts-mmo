use dioxus::prelude::*;

use crate::{
    api::v1::status::ServerStatus,
    constants::css::{self},
    interface::{app::APPLICATION_STATE, widget::audible_button::AudibleButton},
};
// use crate::api::v1::status::handler::call_get_status;

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
    let refresh_status = move |_| {
        // let mut http_client: ureq::Agent = ureq::AgentBuilder::new().build();
        // if let Some(status) = call_get_status(&mut http_client) {
        //     if status.data.status == "online" {
        //         APPLICATION_STATE.write().artifacts_server_status = ServerStatus::Online;
        //     } else {
        //         APPLICATION_STATE.write().artifacts_server_status = ServerStatus::Offline;
        //     }
        //     APPLICATION_STATE.write().sever_status.sync_now(status.data);
        // }
    };

    rsx! {
        div { class: css::CANVAS,
            div { class: css::CANVAS,
                div {
                    label { "Server Status" }
                    AudibleButton { onclick: refresh_status, tooltip: "Refresh status".to_string(),
                        img {
                            class: css::IMAGE_ICON,
                            src: "assets/images/refresh.png"
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

pub async fn check_server_status(
    mut server_status: Signal<String>,
    mut next_server_status_refresh: Signal<String>,
) {
    let mut last_updated: tokio::time::Instant = tokio::time::Instant::now();
    let mut time_to_update: bool = true;
    // loop {
    //     match time_to_update {
    //         true => {
    //             let mut http_client: ureq::Agent = ureq::AgentBuilder::new().build();
    //             match call_get_status(&mut http_client) {
    //                 Some(status) => {
    //                     if status.data.status == "online" {
    //                         APPLICATION_STATE.write().artifacts_server_status =
    //                             ServerStatus::Online;
    //                     } else {
    //                         APPLICATION_STATE.write().artifacts_server_status =
    //                             ServerStatus::Offline;
    //                     }
    //                     server_status.set(format!(
    //                     "Version: {}\nStatus: {}\nCharacters Online: {}\nLast Wipe: {}\nNext Wipe: {}\nAnnouncements: {}",
    //                     status.data.version,
    //                     status.data.status,
    //                     status.data.characters_online,
    //                     status.data.last_wipe,
    //                     status.data.next_wipe,
    //                     serde_json::to_string_pretty(&status.data.announcements).unwrap_or("None".to_string())
    //                 ));
    //                 }
    //                 None => server_status.set("Offline".to_string()),
    //             }
    //             last_updated = tokio::time::Instant::now();
    //             time_to_update = false;
    //             tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //         }
    //         false => match (tokio::time::Instant::now() - last_updated)
    //             >= std::time::Duration::from_secs(60 * 60)
    //         {
    //             true => time_to_update = true,
    //             false => {
    //                 next_server_status_refresh.set(
    //                     (std::time::Duration::from_secs(60 * 60)
    //                         - (tokio::time::Instant::now() - last_updated))
    //                         .as_secs()
    //                         .to_string(),
    //                 );
    //                 tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //             }
    //         },
    //     }
    // }
}
