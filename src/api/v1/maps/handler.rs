use log::{error, info};

use crate::{api::v1::maps::GetAllMapsResponse, constants::api::ARTIFACTS_MMO_HOST};

use super::{Map, GET_ALL_MAPS};

pub fn get_all_maps(http_client: &mut ureq::Agent, api_token: &String) -> Option<Vec<Map>> {
    let url: String = format!("{}{}{}", "https://", ARTIFACTS_MMO_HOST, GET_ALL_MAPS.path);
    let result: Result<ureq::Response, ureq::Error> = http_client
        .get(&url)
        .set("accept", "application/json")
        .set("authorization", &format!("Bearer {}", api_token))
        .call();

    match result {
        Ok(response) => {
            info!("{:?}", response);
            let map_tiles: std::result::Result<GetAllMapsResponse, std::io::Error> =
                response.into_json::<GetAllMapsResponse>();
            match map_tiles {
                Ok(map) => {
                    info!("{:?}", map);
                    info!("{}", serde_json::to_string(&map).unwrap());
                    return Some(map.data);
                }
                Err(error) => error!("{:?}", error),
            }
        }
        Err(err) => error!("{:?}", err),
    }
    None
}
