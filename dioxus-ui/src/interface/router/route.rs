use crate::interface::{
    component::layout::Layout, page::characters::Characters, page::home::HomeComponent,
    page::map::MapComponent, page::monsters::MonstersComponent, page::settings::SettingsComponent,
};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    HomeComponent {},
    #[route("/characters")]
    Characters {},
    #[route("/monsters")]
    MonstersComponent {},
    #[route("/map")]
    MapComponent {},
    #[route("/settings")]
    SettingsComponent {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "The page or resource you requested was not found." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
