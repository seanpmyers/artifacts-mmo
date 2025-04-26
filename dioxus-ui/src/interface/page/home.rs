use dioxus::prelude::*;

use crate::constants::css;

#[component]
pub fn HomeComponent() -> Element {
    // let mut api_key: Signal<String> =
    //     use_synced_storage::<LocalStorage, String>("action_queue".to_string(), String::new);

    rsx! {
        div {
            class: css::CANVAS,
            "Show each character and then allow for an action queue to be edited"
        }
    }
}
