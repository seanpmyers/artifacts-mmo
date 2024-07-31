use anyhow::Result;

use configuration::configure_logging;
use constants::environment_variables::API_TOKEN;

use controller::start_playing;
use log::info;

pub mod api;
pub mod configuration;
pub mod constants;
pub mod controller;
pub mod http;

fn main() -> Result<()> {
    configure_logging()?;
    info!("RUN STARTED");

    let api_token: String =
        std::env::var(API_TOKEN).expect("Environment variable not set: API_TOKEN");

    let mut http_client: ureq::Agent = ureq::AgentBuilder::new().build();

    start_playing(&mut http_client, api_token);

    Ok(())
}
