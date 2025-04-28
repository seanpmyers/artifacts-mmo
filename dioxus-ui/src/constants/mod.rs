pub mod css;

pub mod api {
    pub const ARTIFACTS_MMO_HOST: &str = "api.artifactsmmo.com";
}

pub mod environment_variables {
    pub const API_TOKEN: &str = "API_TOKEN";
}

pub const API_KEY_STORAGE_KEY: &str = "api_key";

pub const ARTIFACTS_MMO: &str = "Artifacts-MMO";

pub const BUTTON_CLICK_SOUND_PATH: &str = "assets/sound/navigation_hover-tap.wav";
pub const BUTTON_CLICK_SOUND: &str = "button_click";

pub const HERO_SIMPLE_CELEBRATION_03_PATH: &str = "assets/sound/hero_simple-celebration-03.wav";
pub const HERO_SIMPLE_CELEBRATION_03_SOUND: &str = "hero_simple_celebration_03";

pub const NAVIGATION_BACKWARD_PATH: &str = "assets/sound/navigation_backward-selection-minimal.wav";
pub const NAVIGATION_BACKWARD_SOUND: &str = "navigation_backward";
pub const NAVIGATION_FORWARD_PATH: &str = "assets/sound/navigation_forward-selection-minimal.wav";
pub const NAVIGATION_FORWARD_SOUND: &str = "navigation_forward";

pub const AUDIO_PATHS: [(&str, &str); 4] = [
    (BUTTON_CLICK_SOUND_PATH, BUTTON_CLICK_SOUND),
    (
        HERO_SIMPLE_CELEBRATION_03_PATH,
        HERO_SIMPLE_CELEBRATION_03_SOUND,
    ),
    (NAVIGATION_BACKWARD_PATH, NAVIGATION_BACKWARD_SOUND),
    (NAVIGATION_FORWARD_PATH, NAVIGATION_FORWARD_SOUND),
];
