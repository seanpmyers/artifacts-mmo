use log::{error, info, warn};
use url::Url;

use crate::{
    api::v1::{
        my_characters::{ActionCraftingResponse, ActionGatheringResponse, GetMyCharactersResponse},
        QueryParameters,
    },
    constants::api::ARTIFACTS_MMO_HOST,
    http::{get_headers, make_api_call},
};

use super::{
    ActionCraftingRequest, ActionGatheringRequest, ActionGeSellItemRequest,
    ActionGeSellItemResponse, ActionMoveRequest, ActionMoveResponse, MyCharacters, ACTION_CRAFTING,
    ACTION_GE_SELL_ITEM, GET_MY_CHARACTERS,
};

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
        ActionMoveRequest::get_path(character_name.to_string())
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
            info!("{}", serde_json::to_string(&action_move).unwrap());
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

pub fn call_get_my_characters(
    http_client: &mut ureq::Agent,
    api_token: &String,
) -> Option<Vec<MyCharacters>> {
    let my_characters_url: String = format!(
        "{}{}{}",
        "https://", ARTIFACTS_MMO_HOST, GET_MY_CHARACTERS.path
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
                    info!("{}", serde_json::to_string(&my_characters).unwrap());
                    return Some(my_characters.data);
                }
                Err(error) => error!("{:?}", error),
            }
        }
        Err(err) => error!("{:?}", err),
    }
    None
}

pub fn call_action_gathering(
    http_client: &mut ureq::Agent,
    api_token: &String,
    character_name: String,
    body: &ActionGatheringRequest,
) -> Option<ActionGatheringResponse> {
    let url: String = format!(
        "{}{}{}",
        "https://",
        ARTIFACTS_MMO_HOST,
        ActionGatheringRequest::get_path(character_name.to_string())
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
            info!("{}", serde_json::to_string(&response_data).unwrap());
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
        ActionCraftingRequest::get_path(character_name)
    ))
    .unwrap();
    let response: Option<ureq::Response> = make_api_call(
        http_client,
        get_headers(api_token),
        ACTION_CRAFTING.http_request_method,
        url,
        Some(body),
    );
    if let Some(response) = response {
        info!("{:?}", response);
        let response_data: ActionCraftingResponse =
            response.into_json::<ActionCraftingResponse>().unwrap();
        info!("{}", serde_json::to_string(&response_data).unwrap());
        return Some(response_data);
    }
    None
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
        ActionGeSellItemRequest::get_path(character_name)
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
        info!("{}", serde_json::to_string(&response_data).unwrap());
        return Some(response_data);
    }

    None
}
