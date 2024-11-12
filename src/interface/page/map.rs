use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

use crate::{
    api::v1::{
        maps::{handler::get_all_maps, Map},
        resources::{get_image_url, ImageResourceType},
    },
    constants::css,
    interface::app::{ApplicationState, APPLICATION_STATE},
};

#[component]
pub fn Map() -> Element {
    let api_key: Signal<String> =
        use_synced_storage::<LocalStorage, String>("api_key".to_string(), String::new);

    use_future(move || async move {
        if APPLICATION_STATE().map_tiles.is_out_of_sync() {
            get_map_tiles(&api_key(), &mut APPLICATION_STATE.signal()).await;
        }
    });

    rsx! {
        div { class: css::CANVAS,
            h1 { class: css::ARTIFACTS_HEADER, "Map" }
            div { class: css::MAP,
                for tile in APPLICATION_STATE().map_tiles.data.map_or(Vec::new(), |map_tiles| map_tiles) {
                    MapTile { tile }
                }
            }
        }
    }
}

#[component]
pub fn MapTile(tile: Map) -> Element {
    rsx! {
       div {
           div {
               div { "{tile.name}" }
               div { "x: {tile.x} y: {tile.y}" }
           }
           img { src: get_image_url(tile.skin, ImageResourceType::Maps), class: css::MAP_TILE}
       }
    }
}

pub async fn get_map_tiles(api_key: &str, app_state: &mut Signal<ApplicationState>) {
    let mut http_client: ureq::Agent = ureq::AgentBuilder::new().build();
    if let Some(map_tiles) = get_all_maps(&mut http_client, &api_key.to_string()) {
        app_state.write().map_tiles.sync_now(map_tiles);
    }
}
