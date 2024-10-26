use dioxus::prelude::*;

use crate::{
    api::v1::{
        maps::Map,
        resources::{get_image_url, ImageResourceType},
    },
    constants::css,
    interface::app::{ApplicationState, APPLICATION_STATE},
};

#[component]
pub fn Map() -> Element {
    let tiles: Signal<Vec<Map>> = use_signal(Vec::new);

    // use_future(move || async move {
    //     if APPLICATION_STATE().map_tiles.is_out_of_sync() {
    //         get_user_characters(
    //             &api_key(),
    //             &mut APPLICATION_STATE.signal(),
    //             &mut last_updated,
    //         )
    //         .await;
    //     }
    // });

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
    rsx! {
        img { src: get_image_url(tile.skin, ImageResourceType::Maps), class: css::MAP_TILE}
    }
}

pub async fn get_map_tiles(api_key: &str, app_state: &mut Signal<ApplicationState>) {
    // let mut http_client: ureq::Agent = ureq::AgentBuilder::new().build();
    // if let Some(my_characters) = call_get_my_characters(&mut http_client, &api_key.to_string()) {
    //     app_state.write().characters = my_characters;
    //     last_updated.set(Local::now());
    // }
}
