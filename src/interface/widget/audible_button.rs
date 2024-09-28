use dioxus::prelude::*;

use crate::interface::app::play_button_click_sound;

#[derive(PartialEq, Clone, Props)]
pub struct AudibleButtonProps {
    onclick: Option<EventHandler<MouseEvent>>,
    class: Option<String>,
    dangerous_inner_html: Option<String>,
    tooltip: Option<String>,
    children: Element,
}

#[component]
pub fn AudibleButton(props: AudibleButtonProps) -> Element {
    match props.dangerous_inner_html {
        Some(danger) => rsx! {
            button {
                onclick: move |evt| {
                    if let Some(handler) = props.onclick {
                        handler.call(evt);
                    }
                    play_button_click_sound();
                },
                class: props.class,
                title: props.tooltip,
                dangerous_inner_html: danger
            }
        },
        None => rsx! {
            button {
                onclick: move |evt| {
                    if let Some(handler) = props.onclick {
                        handler.call(evt);
                    }
                    play_button_click_sound();
                },
                title: props.tooltip,
                class: props.class,
                {props.children}
            }
        },
    }
}
