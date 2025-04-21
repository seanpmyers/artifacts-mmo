use dioxus::prelude::*;

use crate::constants::css::{ARTIFACTS_HEADER, CANVAS};

#[component]
pub fn Monsters() -> Element {
    rsx! {
			div { class: CANVAS,
				h1 { class: ARTIFACTS_HEADER, "Monsters" }
			}
		}
}
