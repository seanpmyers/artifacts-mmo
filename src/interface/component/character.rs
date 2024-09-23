use dioxus::prelude::*;

use crate::api::v1::my_characters::MyCharacters;

#[component]
pub fn Character(character: MyCharacters) -> Element {
    let thing = serde_json::to_string_pretty(&character).unwrap_or("None".to_string());
    rsx! {
			div { class: "canvas",
				p { "{thing}" }
			}
		}
}
