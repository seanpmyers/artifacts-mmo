use dioxus::prelude::*;

use crate::{
    constants::css::{BUTTON, IMAGE_ICON, SIDE_NAV, SIDE_NAV_CONTAINER, SIDE_NAV_ITEM},
    interface::{app::play_button_click_sound, router::route::Route},
};

const CHICKEN_IMAGE: Asset = asset!("/assets/images/chicken.png");
const ASH_TREE_IMAGE: Asset = asset!("/assets/images/ash_tree.png");
const CULTIST_EMPORER_IMAGE: Asset = asset!("/assets/images/cultist_emperor.png");
const MEN_1_IMAGE: Asset = asset!("/assets/images/men1.png");

#[derive(Clone, PartialEq)]
pub struct SideNavigationItem {
    pub route: Route,
    pub link_css: String,
    pub image_src: Asset,
    pub title: &'static str,
}

#[component]
pub fn SideNavigation() -> Element {
    let link_css: [&str; 2] = [BUTTON, SIDE_NAV_ITEM];
    let navigation_items: [SideNavigationItem; 4] = [
        SideNavigationItem {
            route: Route::Home {},
            link_css: link_css.join(" "),
            image_src: CHICKEN_IMAGE,
            title: "Home",
        },
        SideNavigationItem {
            route: Route::Characters {},
            link_css: link_css.join(" "),
            image_src: MEN_1_IMAGE,
            title: "Characters",
        },
        SideNavigationItem {
            route: Route::Monsters {},
            link_css: link_css.join(" "),
            image_src: CULTIST_EMPORER_IMAGE,
            title: "Monsters",
        },
        SideNavigationItem {
            route: Route::MapWidget {},
            link_css: link_css.join(" "),
            image_src: ASH_TREE_IMAGE,
            title: "Map",
        },
    ];
    rsx! {
        div { class: SIDE_NAV,
            div { class: SIDE_NAV_CONTAINER,
                for item in navigation_items {
                    SideNavigationLink { item }
                }
            }
        }
    }
}

#[component]
pub fn SideNavigationLink(item: SideNavigationItem) -> Element {
    rsx! {
        Link {
            class: item.link_css,
            to: item.route,
            onclick: |_| { play_button_click_sound() },
            img { class: IMAGE_ICON, src: item.image_src }
            span { "{item.title}" }
        }
    }
}
