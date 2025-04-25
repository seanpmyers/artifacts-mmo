use artifacts_mmo::api::v4::maps::Map;
use artifacts_mmo::api::v4::resources::ImageResourceType;
use artifacts_mmo::api::{v4, Endpoint};
use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

use crate::interface::app::HTTP_CLIENT;
use crate::{
    constants::css,
    interface::app::{ApplicationState, APPLICATION_STATE},
};

#[component]
pub fn MapWidget() -> Element {
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
           img { src:ImageResourceType::Maps.to_uri_string(&tile.skin), class: css::MAP_TILE}
       }
    }
}

pub async fn get_map_tiles(api_key: &str, app_state: &mut Signal<ApplicationState>) {
    let mut map_tiles = Vec::new();
    let first_page = 1u32;
    let mut request = v4::maps::GetAllMapsRequest {
        content_code: None,
        content_type: None,
        page_size: Some(artifacts_mmo::api::PAGE_SIZE_MAX),
        page: Some(first_page),
    };
    let client = &mut HTTP_CLIENT.write();
    match request.call(client, api_key.to_string()) {
        artifacts_mmo::api::EndpointResponse::Error => log::error!("Failed to get map tiles."),
        artifacts_mmo::api::EndpointResponse::Success(mut response) => {
            map_tiles.append(&mut response.data);
            if response.pages.eq(&first_page) {
                app_state.write().map_tiles.sync_now(map_tiles);
                return;
            }

            for page in (first_page + 1)..response.pages.saturating_add(1) {
                request.page = Some(page);
                match request.call(client, api_key.to_string()) {
                    artifacts_mmo::api::EndpointResponse::Error => {
                        log::error!("Failed to get map tiles.Page: {}", page)
                    }
                    artifacts_mmo::api::EndpointResponse::Success(mut response) => {
                        map_tiles.append(&mut response.data)
                    }
                }
            }
        }
    };

    app_state.write().map_tiles.sync_now(map_tiles);
}
