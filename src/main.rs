use anyhow::Result;

use configuration::configure_logging;

use interface::configuration::desktop::configure_dioxus_desktop;
use log::info;

use dioxus::{desktop::Config, prelude::*};
use tracing::Level;

pub mod api;
pub mod configuration;
pub mod constants;
pub mod controller;
pub mod http;
pub mod interface;

fn main() -> Result<()> {
    configure_logging()?;
    info!("RUN STARTED");

    dioxus_logger::init(Level::DEBUG).expect("failed to initialize logger");
    dioxus_sdk::storage::set_dir!();
    let launch_builder: LaunchBuilder<Config> = LaunchBuilder::desktop()
        .with_cfg(configure_dioxus_desktop().with_background_color((0, 0, 0, 0)));
    launch_builder.launch(interface::app::App);

    Ok(())
}
