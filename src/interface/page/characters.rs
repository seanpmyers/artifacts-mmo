use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

use crate::api::v1::my_characters::{handler::call_get_my_characters, MyCharacters};
use crate::interface::component::character::Character;

pub const CANVAS_CLASS: &str = "canvas";

#[component]
pub fn Characters() -> Element {
    let api_key: Signal<String> =
        use_synced_storage::<LocalStorage, String>("api_key".to_string(), String::new);

    let mut characters: Signal<Vec<MyCharacters>> = use_signal(std::vec::Vec::new);

    use_future(move || async move {
        get_user_characters(&api_key(), &mut characters).await;
    });

    rsx! {
        div { class: CANVAS_CLASS,
            h1 { class: "artifacts-header", "Characters" }
            div {
                for character in characters() {
                    Character { character }
                }
            }
        }
    }
}

pub async fn get_user_characters(api_key: &str, characters: &mut Signal<Vec<MyCharacters>>) {
    let mut http_client: ureq::Agent = ureq::AgentBuilder::new().build();
    if let Some(my_characters) = call_get_my_characters(&mut http_client, &api_key.to_string()) {
        characters.set(my_characters)
    }
}
