use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

use crate::{
    constants::css::{CANVAS, IMAGE_ICON},
    interface::widget::audible_button::AudibleButton,
};

#[component]
pub fn Account() -> Element {
    let mut api_key: Signal<String> =
        use_synced_storage::<LocalStorage, String>("api_key".to_string(), String::new);

    let mut hidden: Signal<bool> = use_signal(|| true);
    let mut editing: Signal<bool> = use_signal(|| false);

    rsx! {
        div { class: CANVAS,
            div {
                label { "API Key" }
                if !editing() {
                    AudibleButton { onclick: move |_| {editing.set(true);}, tooltip: "Edit".to_string(),
                        img { class: IMAGE_ICON, src: asset!("assets/images/edit.png") }
                    }
                } else {
                    AudibleButton { onclick: move |_| editing.set(false), tooltip: "Save".to_string(),
                        img { class: IMAGE_ICON, src: asset!("assets/images/save.png") }
                    }
                }
                if hidden() {
                    AudibleButton { onclick: move |_| hidden.set(false), tooltip: "Show".to_string(),
                        img {
                            class: IMAGE_ICON,
                            src: asset!("assets/images/show_password.png")
                        }
                    }
                } else {
                    AudibleButton { onclick: move |_| hidden.set(true), tooltip: "Hide".to_string(),
                        img {
                            class: IMAGE_ICON,
                            src: asset!("assets/images/hide_password.png")
                        }
                    }
                }
            }
            if editing() {
                input {
                    r#type: "text",
                    value: "{api_key}",
                    oninput: move |event| { *api_key.write() = event.value(); }
                }
            } else {
                if hidden() {
                    input {
                        r#type: "password",
                        value: "{api_key}",
                        disabled: hidden
                    }
                } else {
                    p { "{api_key}" }
                }
            }
        }
    }
}
