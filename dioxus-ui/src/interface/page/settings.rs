use dioxus::prelude::*;

use crate::interface::component::{account::Account, status::Status};

#[component]
pub fn SettingsComponent() -> Element {
    rsx! {
        Account {}
        Status {}
    }
}
