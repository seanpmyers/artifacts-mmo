use std::path::{Path, PathBuf};

use dioxus::{
    desktop::{tao::window::Icon, Config, WindowBuilder},
    prelude::*,
};
use tracing::info;

use crate::constants::ARTIFACTS_MMO;

const MAIN_CSS: Asset = asset!(
    "/assets/main.css",
    ImageAssetOptions::new().with_preload(true)
);

pub fn configure_dioxus_desktop() -> Config {
    let head_html = format!(
        r#"<link rel="stylesheet" href="{}""#,
        MAIN_CSS.resolve().display()
    );
    let icon = asset!("/assets/logo.png");
    let icon_path: &Path = Path::new("assets/logo.png");
    info!(
        "Icon exists: {} {}",
        icon_path.exists(),
        icon.resolve().exists()
    );
    Config::new()
        .with_window(
            WindowBuilder::new()
                .with_transparent(true)
                .with_decorations(true)
                .with_always_on_top(false)
                .with_title(ARTIFACTS_MMO),
        )
        .with_custom_head(head_html)
        .with_icon(load_icon(icon_path.to_path_buf()))
}

fn load_icon(path: PathBuf) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba: Vec<u8> = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
