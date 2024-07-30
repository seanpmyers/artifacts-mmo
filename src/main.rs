use std::{
    fs::{File, OpenOptions},
    i32::MIN,
    io::Write,
    path::Path,
    thread::sleep,
    time::Duration,
};

use anyhow::Result;

use api::v1::{
    characters::Character,
    my_characters::{
        ActionGatheringRequest, ActionGatheringResponse, ActionMoveRequest, ActionMoveResponse,
        GetMyCharactersResponse, MyCharacters,
    },
    status::StatusResponse,
    QueryParameters,
};
use constants::{api::ARTIFACTS_MMO_HOST, environment_variables::API_TOKEN};
use env_logger::{Builder, Target};
use log::{error, info, warn};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub mod api;
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

pub fn write_to_output_file(text: String) {
    let file_path = Path::new("output.txt");
    if !file_path.exists() {
        File::create(file_path).expect("Unable to create file");
    }
    let mut file: File = OpenOptions::new().append(true).open(file_path).unwrap();
    write!(file, "\n{}", text).unwrap();
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

pub fn configure_logging() -> Result<()> {
    Builder::new()
        .target(Target::Stdout)
        .format(move |buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "[ARTIFACTS_MMO]:[{}]:[{}] - {}",
                record.level(),
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Debug)
        .init();

    Ok(())
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
        if character.cooldown > 0 {
            sleep(Duration::from_secs(character.cooldown as u64));
        }
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
        }

        if character.is_inventory_full() {
            *mining_active = false;
            let move_result = call_action_move(
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
                let crafting_result = call_action_crafting();
            }
        }
    }
}

pub fn call_action_crafting() {}
