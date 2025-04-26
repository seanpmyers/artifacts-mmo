use artifacts_mmo::api::{
    v4::{self, monsters::Monster, resources::ImageResourceType},
    Endpoint, PageInput, PAGE_SIZE_MAX,
};
use dioxus::prelude::*;

use crate::{
    constants::css::{self, ARTIFACTS_HEADER, CANVAS},
    interface::app::{APPLICATION_STATE, HTTP_CLIENT},
};

#[component]
pub fn MonstersComponent() -> Element {
    use_effect(move || {
        spawn(async move {
            if APPLICATION_STATE().monsters.is_out_of_sync() {
                get_monsters().await;
            }
        });
    });

    rsx! {
        div { class: CANVAS,
            h1 { class: ARTIFACTS_HEADER, "Monsters" }
            if APPLICATION_STATE().monsters.data.is_some_and(|x| !x.is_empty()) {
                div {
                    for monster in APPLICATION_STATE().monsters.data.map_or(Vec::new(), |monsters| monsters) {
                        MonsterComponent { monster }
                    }
                }
            }
            else { div { "Loading..." } }
        }
    }
}

#[component]
pub fn MonsterComponent(monster: Monster) -> Element {
    rsx! {
       div {
           div {
               div { "{monster.name}" }
               div { "Level: {monster.level} HP: {monster.hp}" }
           }
           img { src:ImageResourceType::Monsters.to_uri_string(&monster.code), class: css::CHARACTER_IMAGE}
       }
    }
}

pub async fn get_monsters() {
    let request = v4::monsters::GetAllMonstersRequest {
        drop: None,
        max_level: None,
        min_level: None,
        page_input: PageInput {
            page: None,
            size: Some(PAGE_SIZE_MAX),
        },
    };

    match request.call(
        &mut HTTP_CLIENT.write(),
        APPLICATION_STATE().api_token.clone(),
    ) {
        artifacts_mmo::api::EndpointResponse::Error => {
            log::error!("Failed to get list of all monsters")
        }
        artifacts_mmo::api::EndpointResponse::Success(response) => {
            APPLICATION_STATE.write().monsters.sync_now(response.data);
        }
    };
}
