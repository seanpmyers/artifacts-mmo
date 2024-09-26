use dioxus::prelude::*;

use crate::interface::app::APPLICATION_STATE;
use crate::interface::icon::MUTED_SVG;
use crate::{constants::css::ICON, interface::icon::SOUND_ON_SVG};

#[component]
pub fn Sound() -> Element {
    rsx! {
        button {
            onclick: move |_| {
                APPLICATION_STATE.write().sound_on = !APPLICATION_STATE().sound_on;
            },
            class: ICON,
            dangerous_inner_html: match APPLICATION_STATE().sound_on {
                true => SOUND_ON_SVG,
                false => MUTED_SVG,
            }
        }
    }
}
