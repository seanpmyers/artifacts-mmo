use dioxus::prelude::*;

use crate::{
    constants::css::{ICON, IMAGE_ICON},
    interface::{app::APPLICATION_STATE, widget::audible_button::AudibleButton},
};

#[component]
pub fn Sound() -> Element {
    rsx! {
        AudibleButton {
            onclick: move |_| {
                APPLICATION_STATE.write().sound_on = !APPLICATION_STATE().sound_on;
            },
            class: ICON,
            tooltip: match APPLICATION_STATE().sound_on {
                true => "Mute audio".to_string(),
                false => "Unmute audio".to_string(),
            },
            if APPLICATION_STATE().sound_on {
                img { class: IMAGE_ICON, src: "assets/images/sound_on.png" }
            } else {
                img { class: IMAGE_ICON, src: "assets/images/muted.png" }
            }
        }
    }
}
