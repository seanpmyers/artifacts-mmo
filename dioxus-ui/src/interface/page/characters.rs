use artifacts_mmo::api::v4::characters::Character;
use artifacts_mmo::api::Endpoint;
use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

use crate::constants::css::{self, ARTIFACTS_HEADER, CANVAS, MY_CHARACTERS};
use crate::interface::app::{ApplicationState, APPLICATION_STATE, HTTP_CLIENT};
use crate::interface::component::character::Character as CharacterComponent;
use crate::interface::widget::audible_button::AudibleButton;

#[component]
pub fn Characters() -> Element {
    let api_key: Signal<String> =
        use_synced_storage::<LocalStorage, String>("api_key".to_string(), String::new);

    let mut search: Signal<String> = use_signal(String::new);

    let visible_characters: Memo<Vec<Character>> = use_memo(move || {
        let filtered_characters: Vec<Character> = APPLICATION_STATE
            .read()
            .characters
            .data
            .clone()
            .map_or(Vec::new(), |characters| characters)
            .iter()
            .filter(|charater| {
                charater
                    .profile
                    .name
                    .to_lowercase()
                    .contains(&search().to_lowercase())
            })
            .cloned()
            .collect::<Vec<_>>();

        filtered_characters
    });

    use_effect(move || {
        spawn(async move {
            update_characters(&api_key()).await;
        });
    });

    rsx! {
        div { class: CANVAS,
            h1 { class: ARTIFACTS_HEADER, "Characters" }
            div {
                label { "Search" }
                input {
                    value: search,
                    r#type: "text",
                    onchange: move |event| {
                        search.set(event.value());
                    }
                }
                AudibleButton { onclick: move |_|{
                    spawn(async move {
                       update_characters(&api_key()).await
                    });
                }, tooltip: "Refresh characters".to_string(),
                    img {
                            class: css::IMAGE_ICON,
                            src: asset!("assets/images/refresh.png")
                     }
                }
            }
            div { class: MY_CHARACTERS,
                for character in visible_characters() {
                    CharacterComponent { character }
                }
            }
        }
    }
}

pub async fn get_user_characters(api_key: &str, app_state: &mut Signal<ApplicationState>) {
    let request = artifacts_mmo::api::v4::my_characters::GetMyCharactersRequest {};
    let response = request.call(&mut HTTP_CLIENT.write(), api_key.to_string());
    match response {
        artifacts_mmo::api::EndpointResponse::Error => log::error!("Failed to get my characters"),
        artifacts_mmo::api::EndpointResponse::Success(response) => {
            app_state.write().characters.sync_now(response.data);
        }
    }
}

pub async fn update_characters(api_key: &str) {
    if APPLICATION_STATE().characters.is_out_of_sync() {
        get_user_characters(api_key, &mut APPLICATION_STATE.signal()).await;
    }
}
