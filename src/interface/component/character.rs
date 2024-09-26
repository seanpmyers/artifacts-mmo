use dioxus::prelude::*;

use crate::{
    api::v1::my_characters::MyCharacters,
    constants::css::{CHARACTER_IMAGE, ON_CANVAS},
};

#[component]
pub fn Character(character: MyCharacters) -> Element {
    let character_json_string: String =
        serde_json::to_string_pretty(&character).unwrap_or("None".to_string());
    rsx! {
        div { class: ON_CANVAS,
            h3 { "{character.name}" }
            img {
                src: format!("https://artifactsmmo.com/images/characters/{}.png", character.skin),
                class: CHARACTER_IMAGE
            }
            div {
                label { "Level" }
                div { "{character.level}" }
            }
        }
    }
}
