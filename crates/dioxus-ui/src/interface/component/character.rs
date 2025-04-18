use dioxus::{desktop::use_window, prelude::*};

use crate::{
    constants::css::{self},
    interface::{
        app::blur_window, configuration::desktop::configure_dioxus_desktop,
        widget::audible_button::AudibleButton,
    },
};

#[component]
pub fn Character(character: characters::Character) -> Element {
    let character_json_string: String =
        serde_json::to_string_pretty(&character).unwrap_or("None".to_string());
    let character_clone = character.clone();

    rsx! {
        div { class: format!("{} {}", css::ON_CANVAS, css::CHARACTER),
            div {
                h3 { "{character.name}" }
                AudibleButton {
                    onclick: move |_: MouseEvent| {
                        dioxus::desktop::window()
                            .new_window(
                                VirtualDom::new_with_props(
                                    character_json_window,
                                    character_json_string.clone(),
                                ),
                                configure_dioxus_desktop().with_background_color((0, 0, 0, 0)),
                            );
                    },
                    "JSON"
                }
                AudibleButton{
                    onclick: move |_: MouseEvent| {
                        dioxus::desktop::window()
                            .new_window(
                                VirtualDom::new_with_props(character_window, character_clone.clone()),
                                configure_dioxus_desktop().with_background_color((0, 0, 0, 0)),
                            );
                    },
                    "View"
                }
            }
            img {
                src: get_image_url(character.skin.to_string(), ImageResourceType::Characters),
                class: css::CHARACTER_IMAGE
            }
            div {
                div { "x: {character.x} y: {character.y}"}
                div { "" }
            }
        }
    }
}

fn character_json_window(json: String) -> Element {
    blur_window(&use_window().window);
    rsx! {
        div {
            h1 { class: css::ARTIFACTS_HEADER, "JSON" }
            div { class: css::CANVAS,
                pre {
                    code { class: css::CODE, language: "json", "{json}" }
                }
            }
        }
    }
}

fn character_window(character: characters::Character) -> Element {
    blur_window(&use_window().window);
    let item_img_url = |code: String| {
        if code.is_empty() {
            return None;
        }
        Some(get_image_url(code, ImageResourceType::Items))
    };
    rsx! {
        div { class: css::CHARACTER_WINDOW,
            h1 { class: css::ARTIFACTS_HEADER, "{character.name}" }
            div {
                label { "Level" }
                span { "{character.level}" }
            }
            div { class: css::CHARACTER_VIEW,
                CharacterViewBlock {
                    label: "Helmet",
                    css: format!("{} {}", css::EQUIP_LEFT, css::EQUIPMENT_BLOCK),
                    url: item_img_url(character.helmet_slot)
                }
                div { class: format!("{}", css::CHARACTER_VIEW_IMAGE),
                    img { src: get_image_url(character.skin.to_string(), ImageResourceType::Characters) }
                }
                CharacterViewBlock {
                    label: "Shield",
                    css: format!("{} {}", css::EQUIP_RIGHT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Amulet",
                    css: format!("{} {}", css::EQUIP_LEFT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Weapon",
                    css: format!("{} {}", css::EQUIP_RIGHT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Body Armor",
                    css: format!("{} {}", css::EQUIP_LEFT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Ring 1",
                    css: format!("{} {}", css::EQUIP_RIGHT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Leg Armor",
                    css: format!("{} {}", css::EQUIP_LEFT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Ring 2",
                    css: format!("{} {}", css::EQUIP_RIGHT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Boots",
                    css: format!("{} {}", css::EQUIP_LEFT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Consumable 1",
                    css: format!("{} {}", css::EQUIP_RIGHT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Artifact 1",
                    css: format!("{} {}", css::EQUIP_LEFT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Consumable 2",
                    css: format!("{} {}", css::EQUIP_RIGHT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Artifact 2",
                    css: format!("{} {}", css::EQUIP_LEFT, css::EQUIPMENT_BLOCK)
                }
                CharacterViewBlock {
                    label: "Artifact 3",
                    css: format!("{} {}", css::EQUIP_LEFT, css::EQUIPMENT_BLOCK)
                }
            }
        }
    }
}

#[component]
pub fn CharacterViewBlock(label: &'static str, css: String, url: Option<String>) -> Element {
    rsx!(
        div { class: css,
            label { "{label}" }
            if url.is_some() {
                div {
                    img { src: url, class: css::CHARACTER_IMAGE }
                }
            } else {
                div { class: css::SUBTLE_TEXT, "Slot empty" }
            }
        }
    )
}
