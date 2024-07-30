use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use anyhow::Result;

use api::v1::{
    my_characters::{ActionMoveRequest, ActionMoveResponse, GetMyCharactersResponse, MyCharacters},
    status::StatusResponse,
    QueryParameters,
};
use constants::{api::ARTIFACTS_MMO_HOST, environment_variables::API_TOKEN};
use env_logger::{Builder, Target};
use log::{error, info};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub mod api;
pub mod constants;
pub mod http;

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
        if !characters.is_empty() {
            let possible_character: Option<&MyCharacters> = characters.first();
            if let Some(first) = possible_character {
                let movement: ActionMoveRequest = ActionMoveRequest {
                    x: first.x + 2,
                    y: first.y,
                };
                call_action_move(&mut http_client, &api_token, &first.name, &movement)
            }
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

pub fn call_get_status(http_client: &mut ureq::Agent) {
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
            info!("{}", serde_json::to_string_pretty(&status).unwrap())
        }
        Err(err) => error!("{:?}", err),
    }
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
            let my_characters: GetMyCharactersResponse =
                response.into_json::<GetMyCharactersResponse>().unwrap();
            info!("{:?}", my_characters);
            info!("{}", serde_json::to_string_pretty(&my_characters).unwrap());
            return Some(my_characters.data);
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
) {
    let my_characters_url: String = format!(
        "{}{}{}",
        "https://",
        ARTIFACTS_MMO_HOST,
        api::v1::my_characters::ActionMove::get_path(character_name.to_string())
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
            info!("{}", serde_json::to_string_pretty(&action_move).unwrap())
        }
        Err(err) => error!("{:?}", err),
    }
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
