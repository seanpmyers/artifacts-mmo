use dioxus::prelude::*;

use crate::interface::component::{account::Account, status::Status};

#[component]
pub fn Home() -> Element {
    rsx! {
        Account {}
        Status {}
    }
}
