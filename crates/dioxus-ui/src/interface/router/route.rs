use crate::interface::{
    component::layout::Layout, page::characters::Characters, page::home::Home, page::map::Map,
    page::monsters::Monsters,
};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/characters")]
    Characters {},
    #[route("/monsters")]
    Monsters {},
    #[route("/map")]
    Map {},
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
