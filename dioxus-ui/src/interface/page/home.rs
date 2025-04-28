use artifacts_mmo::api::{
    v4::{characters::Character, resources::ImageResourceType},
    CharacterActionQueue,
};
use dioxus::prelude::*;

use crate::{constants::css, interface::app::APPLICATION_STATE};

#[component]
pub fn HomeComponent() -> Element {
    let visible_characters: Memo<Vec<Character>> = use_memo(move || {
        let filtered_characters: Vec<Character> = APPLICATION_STATE
            .read()
            .characters
            .data
            .clone()
            .map_or(Vec::new(), |characters| characters)
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        filtered_characters
    });

    let queues: Signal<Vec<CharacterActionQueue>> =
        use_signal(move || match APPLICATION_STATE().characters.data {
            Some(characters) => Character::to_action_queue_list(&characters),
            None => Vec::with_capacity(5),
        });

    rsx! {
        div {
            class: css::CANVAS,
            for character in visible_characters() {
                div { "{character.profile.name}" }
                img {
                    src: ImageResourceType::Characters.to_uri_string(&character.profile.skin.to_string()),
                    class: css::CHARACTER_IMAGE
                }
            }
        }
    }
}
