use chrono::{DateTime, Local, TimeDelta};
use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

use crate::api::v1::my_characters::handler::call_get_my_characters;
use crate::constants::css::{ARTIFACTS_HEADER, CANVAS, MY_CHARACTERS};
use crate::interface::app::{ApplicationState, APPLICATION_STATE};
use crate::interface::component::character::Character;

#[component]
pub fn Characters() -> Element {
    let api_key: Signal<String> =
        use_synced_storage::<LocalStorage, String>("api_key".to_string(), String::new);

    let mut last_updated: Signal<DateTime<Local>> = use_signal(Local::now);

    use_future(move || async move {
        if APPLICATION_STATE().characters.is_empty()
            || Local::now() - last_updated() >= TimeDelta::hours(1)
        {
            get_user_characters(
                &api_key(),
                &mut APPLICATION_STATE.signal(),
                &mut last_updated,
            )
            .await;
        }
    });

    rsx! {
        div { class: CANVAS,
            h1 { class: ARTIFACTS_HEADER, "Characters" }
            div { class: MY_CHARACTERS,
                for character in APPLICATION_STATE().characters {
                    Character { character }
                }
            }
        }
    }
}

pub async fn get_user_characters(
    api_key: &str,
    app_state: &mut Signal<ApplicationState>,
    last_updated: &mut Signal<DateTime<Local>>,
) {
    let mut http_client: ureq::Agent = ureq::AgentBuilder::new().build();
    if let Some(my_characters) = call_get_my_characters(&mut http_client, &api_key.to_string()) {
        app_state.write().characters = my_characters;
        last_updated.set(Local::now());
    }
}
