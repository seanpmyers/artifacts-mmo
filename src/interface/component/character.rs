use dioxus::prelude::*;

use crate::{
    api::v1::{
        my_characters::MyCharacters,
        resources::{get_image_url, ImageResourceType},
    },
    constants::css::{CHARACTER, CHARACTER_IMAGE, ON_CANVAS},
};

#[component]
pub fn Character(character: MyCharacters) -> Element {
    let character_json_string: String =
        serde_json::to_string_pretty(&character).unwrap_or("None".to_string());
    rsx! {
        div { class: format!("{} {}", ON_CANVAS, CHARACTER),
            h3 { "{character.name}" }
            img {
                src: get_image_url(character.skin.to_string(), ImageResourceType::Characters),
                class: CHARACTER_IMAGE
            }
            div {
                label { "Level" }
                div { "{character.level}" }
            }
        }
    }
}
