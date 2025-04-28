#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod configuration;
pub mod constants;
pub mod interface;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    configuration::configure_logs();

    eframe::run_native(
        constants::APPLICATION_TITLE,
        interface::app::configure_eframe_native_options(),
        Box::new(|creation_context| {
            egui_extras::install_image_loaders(&creation_context.egui_ctx);
            Ok(Box::new(interface::app::ApplicationState::new(
                creation_context,
            )))
        }),
    )
}
