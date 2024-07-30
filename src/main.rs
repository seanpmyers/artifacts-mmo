use std::{thread::sleep, time::Duration};

use anyhow::Result;

use api::v1::{
    characters::Character,
    items::{GetItemRequest, GetItemResponse, GET_ITEM},
    my_characters::{
        ActionCraftingRequest, ActionCraftingResponse, ActionGatheringRequest,
        ActionGatheringResponse, ActionGeSellItemRequest, ActionGeSellItemResponse,
        ActionMoveRequest, ActionMoveResponse, GetMyCharactersResponse, MyCharacters,
        ACTION_CRAFTING, ACTION_GE_SELL_ITEM,
    },
    status::StatusResponse,
    QueryParameters,
};
use configuration::configure_logging;
use constants::{api::ARTIFACTS_MMO_HOST, environment_variables::API_TOKEN};

use http::HttpRequestMethod;
use log::{error, info, warn};
use url::Url;

pub mod api;
pub mod configuration;
pub mod constants;
pub mod http;

pub const COPPER_ROCKS_LOCATION: (i32, i32) = (2, 0);
pub const MINING_WORKSHOP_LOCATION: (i32, i32) = (1, 5);
pub const WEAPON_CRAFTING_WORKSHOP_LOCATION: (i32, i32) = (2, 1);
pub const GRAND_EXCHANGE_LOCATION: (i32, i32) = (5, 1);

fn main() -> Result<()> {
    configure_logging()?;
    info!("RUN STARTED");

    let api_token: String =
        std::env::var(API_TOKEN).expect("Environment variable not set: API_TOKEN");

    let mut http_client: ureq::Agent = ureq::AgentBuilder::new().build();

    call_get_status(&mut http_client);
    let my_characters: Option<Vec<MyCharacters>> =
        call_get_my_characters(&mut http_client, &api_token);

    if let Some(characters) = my_characters {
        let mut mining_threads: Vec<std::thread::JoinHandle<()>> = vec![];
        for character in characters.into_iter() {
            info!("Found character: {:?}", character);
            let mut mining_active: bool = true;
            let character: Character = character.to_character();
            let mut new_client: ureq::Agent = http_client.clone();
            let duplicate_token: String = api_token.clone();
            mining_threads.push(std::thread::spawn(move || {
                start_mining(
                    character,
                    &mut new_client,
                    &duplicate_token,
                    &mut mining_active,
                )
            }));
        }
        for thread in mining_threads {
            thread.join().unwrap();
        }
    }

    Ok(())
}

pub fn call_get_status(http_client: &mut ureq::Agent) -> Option<StatusResponse> {
    let status_url: String = format!(
        "{}{}{}",
        "https://",
        ARTIFACTS_MMO_HOST,
        api::v1::status::STATUS.path
    );
    let get_status_result: Result<ureq::Response, ureq::Error> = http_client
        .get(&status_url)
        .set("accept", "application/json")
        .call();
    match get_status_result {
        Ok(response) => {
            let status: StatusResponse = response.into_json::<StatusResponse>().unwrap();
            info!("{:?}", status);
            info!("{}", serde_json::to_string_pretty(&status).unwrap());
            return Some(status);
        }
        Err(err) => error!("{:?}", err),
    }
    None
}

pub fn call_get_my_characters(
    http_client: &mut ureq::Agent,
    api_token: &String,
) -> Option<Vec<MyCharacters>> {
    let my_characters_url: String = format!(
        "{}{}{}",
        "https://",
        ARTIFACTS_MMO_HOST,
        api::v1::my_characters::GET_MY_CHARACTERS.path
    );
    let get_my_characters_result: Result<ureq::Response, ureq::Error> = http_client
        .get(&my_characters_url)
        .set("accept", "application/json")
        .set("authorization", &format!("Bearer {}", api_token))
        .call();

    match get_my_characters_result {
        Ok(response) => {
            info!("{:?}", response);
            let my_characters_result: std::result::Result<GetMyCharactersResponse, std::io::Error> =
                response.into_json::<GetMyCharactersResponse>();
            match my_characters_result {
                Ok(my_characters) => {
                    info!("{:?}", my_characters);
                    info!("{}", serde_json::to_string_pretty(&my_characters).unwrap());
                    return Some(my_characters.data);
                }
                Err(error) => error!("{:?}", error),
            }
        }
        Err(err) => error!("{:?}", err),
    }
    None
}

pub fn call_action_move(
    http_client: &mut ureq::Agent,
    api_token: &String,
    character_name: &str,
    body: &ActionMoveRequest,
) -> Option<ActionMoveResponse> {
    let my_characters_url: String = format!(
        "{}{}{}",
        "https://",
        ARTIFACTS_MMO_HOST,
        api::v1::my_characters::ActionMoveRequest::get_path(character_name.to_string())
    );
    let action_move_result: Result<ureq::Response, ureq::Error> = http_client
        .post(&my_characters_url)
        .set("accept", "application/json")
        .set("authorization", &format!("Bearer {}", api_token))
        .send_json(body);

    match action_move_result {
        Ok(response) => {
            info!("{:?}", response);
            let action_move: ActionMoveResponse =
                response.into_json::<ActionMoveResponse>().unwrap();
            info!("{}", serde_json::to_string_pretty(&action_move).unwrap());
            return Some(action_move);
        }
        Err(error) => match error {
            ureq::Error::Status(code, response) => match code {
                404 => error!("response: {:?}", response),
                486 => warn!("{:?} ", response),
                490 => warn!("response: {:?}", response),
                498 => error!("response: {:?}", response),
                499 => warn!("response: {:?}", response),
                _ => error!(" response: {:?}", response),
            },
            ureq::Error::Transport(_) => error!("{:?}", error),
        },
    }
    None
}

pub fn call_action_gathering(
    http_client: &mut ureq::Agent,
    api_token: &String,
    character_name: &str,
    body: &ActionGatheringRequest,
) -> Option<ActionGatheringResponse> {
    let url: String = format!(
        "{}{}{}",
        "https://",
        ARTIFACTS_MMO_HOST,
        api::v1::my_characters::ActionGatheringRequest::get_path(character_name.to_string())
    );
    let api_request_result: Result<ureq::Response, ureq::Error> = http_client
        .post(&url)
        .set("accept", "application/json")
        .set("authorization", &format!("Bearer {}", api_token))
        .send_json(body);
    match api_request_result {
        Ok(response) => {
            info!("{:?}", response);
            let response_data: ActionGatheringResponse =
                response.into_json::<ActionGatheringResponse>().unwrap();
            info!("{}", serde_json::to_string_pretty(&response_data).unwrap());
            return Some(response_data);
        }
        Err(error) => match error {
            ureq::Error::Status(code, response) => match code {
                404 => error!("response: {:?}", response),
                486 => warn!("{:?} ", response),
                493 => error!("response: {:?}", response),
                498 => error!("response: {:?}", response),
                499 => warn!("response: {:?}", response),
                _ => error!(" response: {:?}", response),
            },
            ureq::Error::Transport(_) => error!("{:?}", error),
        },
    }
    None
}

pub fn start_mining(
    mut character: Character,
    http_client: &mut ureq::Agent,
    api_token: &String,
    mining_active: &mut bool,
) {
    if character.x != 2 || character.y != 0 {
        let movement: ActionMoveRequest = ActionMoveRequest { x: 2, y: 0 };
        let action_move: Option<ActionMoveResponse> =
            call_action_move(http_client, api_token, &character.name, &movement);
        if let Some(action_move_response) = action_move {
            character = action_move_response.data.character;
        }
    }
    while *mining_active {
        character.wait_for_cooldown();
        let gathering_response: Option<ActionGatheringResponse> = call_action_gathering(
            http_client,
            api_token,
            &character.name,
            &ActionGatheringRequest {
                name: character.name.clone(),
            },
        );

        if let Some(gathering) = gathering_response {
            character = gathering.data.character;
            character.wait_for_cooldown();
        }
        let full_inventory: bool = character.is_inventory_full();
        if full_inventory {
            *mining_active = false;
            let move_result: Option<ActionMoveResponse> = call_action_move(
                http_client,
                api_token,
                &character.name,
                &ActionMoveRequest {
                    x: MINING_WORKSHOP_LOCATION.0,
                    y: MINING_WORKSHOP_LOCATION.1,
                },
            );
            if let Some(move_response) = move_result {
                character = move_response.data.character;
                character.wait_for_cooldown();
                let item_data: Option<GetItemResponse> = call_get_item(
                    http_client,
                    api_token,
                    GetItemRequest {
                        code: character.inventory_slot1.clone(),
                    },
                );
                if let Some(item) = item_data {
                    let crafting_result: Option<ActionCraftingResponse> = call_action_crafting(
                        http_client,
                        api_token,
                        character.name.to_string(),
                        ActionCraftingRequest {
                            code: "copper".to_string(),
                            quantity: character.inventory_slot1_quantity / 6,
                        },
                    );
                    if let Some(crafting) = crafting_result {
                        character = crafting.data.character;
                        character.wait_for_cooldown();
                        let move_result: Option<ActionMoveResponse> = call_action_move(
                            http_client,
                            api_token,
                            &character.name,
                            &ActionMoveRequest {
                                x: WEAPON_CRAFTING_WORKSHOP_LOCATION.0,
                                y: WEAPON_CRAFTING_WORKSHOP_LOCATION.1,
                            },
                        );
                        if let Some(movement) = move_result {
                            character = movement.data.character;
                            character.wait_for_cooldown();
                            let crafting_result: Option<ActionCraftingResponse> =
                                call_action_crafting(
                                    http_client,
                                    api_token,
                                    character.name.to_string(),
                                    ActionCraftingRequest {
                                        code: "copper_dagger".to_string(),
                                        quantity: character.inventory_slot2_quantity / 3,
                                    },
                                );
                            if let Some(crafting) = crafting_result {
                                character = crafting.data.character;
                                character.wait_for_cooldown();
                                let move_result: Option<ActionMoveResponse> = call_action_move(
                                    http_client,
                                    api_token,
                                    &character.name,
                                    &ActionMoveRequest {
                                        x: GRAND_EXCHANGE_LOCATION.0,
                                        y: GRAND_EXCHANGE_LOCATION.1,
                                    },
                                );
                                if let Some(movement_to_ge) = move_result {
                                    character = movement_to_ge.data.character;
                                    character.wait_for_cooldown();
                                    let sell_result: Option<ActionGeSellItemResponse> =
                                        call_action_ge_sell_item(
                                            http_client,
                                            api_token,
                                            character.name.to_string(),
                                            ActionGeSellItemRequest {
                                                code: character.inventory_slot3.clone(),
                                                quantity: character.inventory_slot3_quantity,
                                                price: 100f32,
                                            },
                                        );
                                    if let Some(sell) = sell_result {
                                        character = sell.data.character;
                                        character.wait_for_cooldown();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn call_get_item(
    http_client: &mut ureq::Agent,
    api_token: &String,
    body: GetItemRequest,
) -> Option<GetItemResponse> {
    let url: Url = Url::parse(&format!(
        "{}{}{}",
        "https://",
        ARTIFACTS_MMO_HOST,
        api::v1::items::GetItemRequest::get_path(body.code)
    ))
    .unwrap();

    let api_request_result: Option<ureq::Response> = make_api_call::<GetItemRequest>(
        http_client,
        get_headers(api_token),
        GET_ITEM.http_request_method,
        url,
        None,
    );
    if let Some(response) = api_request_result {
        info!("{:?}", response);
        let response_data: GetItemResponse = response.into_json::<GetItemResponse>().unwrap();
        info!("{}", serde_json::to_string_pretty(&response_data).unwrap());
        return Some(response_data);
    }
    None
}

pub fn call_action_crafting(
    http_client: &mut ureq::Agent,
    api_token: &String,
    character_name: String,
    body: ActionCraftingRequest,
) -> Option<ActionCraftingResponse> {
    let url: Url = Url::parse(&format!(
        "{}{}{}",
        "https://",
        ARTIFACTS_MMO_HOST,
        api::v1::my_characters::ActionCraftingRequest::get_path(character_name)
    ))
    .unwrap();
    let reponse: Option<ureq::Response> = make_api_call(
        http_client,
        get_headers(api_token),
        ACTION_CRAFTING.http_request_method,
        url,
        Some(body),
    );
    if let Some(response) = reponse {
        info!("{:?}", response);
        let response_data: ActionCraftingResponse =
            response.into_json::<ActionCraftingResponse>().unwrap();
        info!("{}", serde_json::to_string_pretty(&response_data).unwrap());
        return Some(response_data);
    }
    None
}

pub fn start_working() {}

pub fn make_api_call<T: serde::Serialize>(
    http_client: &mut ureq::Agent,
    headers: Vec<(String, String)>,
    http_request_method: HttpRequestMethod,
    url: Url,
    body: Option<T>,
) -> Option<ureq::Response> {
    let mut request: ureq::Request =
        http_client.request_url(&http_request_method.to_string(), &url);
    for header in headers {
        request = request.set(&header.0, &header.1);
    }
    match body {
        Some(body) => {
            let result = request.send_json(body);
            match result {
                Ok(response) => return Some(response),
                Err(error) => match error {
                    ureq::Error::Status(code, response) => match code {
                        404 => error!("response: {:?}", response),
                        486 => warn!("{:?} ", response),
                        493 => error!("response: {:?}", response),
                        498 => error!("response: {:?}", response),
                        499 => warn!("response: {:?}", response),
                        _ => error!(" response: {:?}", response),
                    },
                    ureq::Error::Transport(_) => error!("{:?}", error),
                },
            };
            None
        }
        None => {
            let result = request.call();
            match result {
                Ok(response) => return Some(response),
                Err(error) => match error {
                    ureq::Error::Status(code, response) => match code {
                        404 => error!("response: {:?}", response),
                        486 => warn!("{:?} ", response),
                        493 => error!("response: {:?}", response),
                        498 => error!("response: {:?}", response),
                        499 => warn!("response: {:?}", response),
                        _ => error!(" response: {:?}", response),
                    },
                    ureq::Error::Transport(_) => error!("{:?}", error),
                },
            };
            None
        }
    }
}

pub fn get_headers(api_token: &String) -> Vec<(String, String)> {
    vec![
        ("accept".to_string(), "application/json".to_string()),
        ("authorization".to_string(), format!("Bearer {}", api_token)),
    ]
}

pub fn call_action_ge_sell_item(
    http_client: &mut ureq::Agent,
    api_token: &String,
    character_name: String,
    body: ActionGeSellItemRequest,
) -> Option<ActionGeSellItemResponse> {
    let url: Url = Url::parse(&format!(
        "{}{}{}",
        "https://",
        ARTIFACTS_MMO_HOST,
        api::v1::my_characters::ActionGeSellItemRequest::get_path(character_name)
    ))
    .unwrap();

    let api_request_result: Option<ureq::Response> = make_api_call::<ActionGeSellItemRequest>(
        http_client,
        get_headers(api_token),
        ACTION_GE_SELL_ITEM.http_request_method,
        url,
        Some(body),
    );

    if let Some(response) = api_request_result {
        info!("{:?}", response);
        let response_data: ActionGeSellItemResponse =
            response.into_json::<ActionGeSellItemResponse>().unwrap();
        info!("{}", serde_json::to_string_pretty(&response_data).unwrap());
        return Some(response_data);
    }

    None
}
