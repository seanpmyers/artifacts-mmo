use dioxus::prelude::*;

use crate::{api::v1::maps::Map, constants::css};

#[component]
pub fn Map() -> Element {
    let tiles: Signal<Vec<Map>> = use_signal(Vec::new);

    rsx! {
        div { class: css::CANVAS,
            h1 { class: css::ARTIFACTS_HEADER, "Map" }
            div {
                for tile in tiles() {
                    MapTile { tile }
                }
            }
        }
    }
}

#[component]
pub fn MapTile(tile: Map) -> Element {
    rsx! {}
}
