use dioxus::prelude::*;

use crate::interface::{
    app::APPLICATION_STATE, router::route::Route, style::theme_toggle::ThemeToggle,
    widget::server_status::ServerStatus,
};

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: format!(
                "{} container",
                APPLICATION_STATE().current_theme.to_string().to_lowercase(),
            ),
            div { class: "top-nav",
                h3 { class: "top-nav-heading", "Artifacts-MMO" }
                ServerStatus {}
                ThemeToggle {}
            }
            div { class: "side-nav",
                div { class: "side-nav-container",
                    Link { class: "button side-nav-item", to: Route::Home {},
                        img {
                            class: "image-icon",
                            src: "assets/images/chicken.png"
                        }
                        span { "Home" }
                    }
                    Link { class: "button side-nav-item", to: Route::Characters {},
                        img {
                            class: "image-icon",
                            src: "assets/images/men1.png"
                        }
                        span { "Characters" }
                    }
                }
            }
            main { class: "", Outlet::<Route> {} }
        }
    }
}
