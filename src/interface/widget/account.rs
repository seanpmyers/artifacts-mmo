use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

pub const CANVAS_CLASS: &str = "canvas";

#[component]
pub fn Account() -> Element {
    let mut api_key: Signal<String> =
        use_synced_storage::<LocalStorage, String>("api_key".to_string(), String::new);

    let mut editing: Signal<bool> = use_signal(|| false);

    rsx! {
			div { class: CANVAS_CLASS,
				div {
					label { "API Key" }
					if !editing() {
						button { onclick: move |_| editing.set(true), "Edit" }
					}
				}
				if editing() {
					input {
						r#type: "text",
						value: "{api_key}",
						oninput: move |event| { api_key.set(event.value()) }
					}
					button { onclick: move |_| editing.set(false), "Save" }
				} else {
					p { "{api_key}" }
				}
			}
		}
}
