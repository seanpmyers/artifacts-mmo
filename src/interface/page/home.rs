use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

use crate::{
    api::v1::status::{handler::call_get_status, StatusResponse},
    interface::app::APPLICATION_STATE,
};

const CANVAS_CLASS: &str = "canvas";

#[component]
pub fn Home() -> Element {
    let mut api_key: Signal<String> =
        use_synced_storage::<LocalStorage, String>("api_key".to_string(), String::new);
    let mut server_status: Signal<String> = use_signal(|| "Offline".to_string());
    let mut next_server_status_refresh: Signal<String> = use_signal(|| "".to_string());

    use_future(move || async move {
        let mut last_updated: tokio::time::Instant = tokio::time::Instant::now();
        let mut time_to_update: bool = true;
        loop {
            match time_to_update {
                true => {
                    let mut http_client: ureq::Agent = ureq::AgentBuilder::new().build();
                    let server_status_option: Option<StatusResponse> =
                        call_get_status(&mut http_client);
                    match server_status_option {
                    Some(status) => server_status.set(format!(
                        "Version: {}\nStatus: {}\nCharacters Online: {}\nLast Wipe: {}\nNext Wipe: {}\nAnnouncements: {}",
                        status.data.version,
                        status.data.status,
                        status.data.characters_online,
                        status.data.last_wipe,
                        status.data.next_wipe,
                        serde_json::to_string_pretty(&status.data.announcements).unwrap_or("None".to_string())
                    )),
                    None => server_status.set("Offline".to_string()),
                }
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
    });

    let refresh_status = move |_| {
        let mut http_client: ureq::Agent = ureq::AgentBuilder::new().build();
        let server_status_option: Option<StatusResponse> = call_get_status(&mut http_client);
        match server_status_option {
            Some(status) => server_status.set(format!(
                "Version: {}\nStatus: {}\nCharacters Online: {}\nLast Wipe: {}\nNext Wipe: {}\nAnnouncements: {}",
                status.data.version,
                status.data.status,
                status.data.characters_online,
                status.data.last_wipe,
                status.data.next_wipe,
                serde_json::to_string_pretty(&status.data.announcements).unwrap_or("None".to_string())
            )),
            None => server_status.set("Offline".to_string()),
        }
    };

    rsx! {
        div { class: CANVAS_CLASS,
            "Current theme: "
            {APPLICATION_STATE().current_theme.to_string()}
        }
        div { class: CANVAS_CLASS,
            div {
                label { "API_KEY" }
            }
            input {
                r#type: "text",
                value: "{api_key}",
                oninput: move |event| { api_key.set(event.value()) }
            }
        }
        div { class: CANVAS_CLASS,
            div {
                label { "Server Status" }
                button { onclick: refresh_status, "Refresh" }
                label { "Next refresh in: {next_server_status_refresh}s" }
            }
            p { "{server_status}" }
        }
    }
}
