use web_sys::MouseEvent;
use yew::{classes, function_component, html, virtual_dom::VNode, Callback, Classes, Properties};

use crate::helpers::colors::Color;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: VNode,
    pub variant: Color,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn component(props: &ButtonProps) -> VNode {
    let classes = classes!(
        [
            "px-4",
            "py-2",
            "rounded-md",
            "transition",
            "ease-in-out",
            "delay-150",
            "duration-300",
            "hover:scale-110",
            "hover:-translate-y-1",
        ],
        props.classes.clone(),
        props.variant.bg_color(),
        match props.variant {
            Color::Warning | Color::Light => "text-dark",
            _ => "text-light",
        }
    );

    html! {
        <button
            class={classes}
            onclick={props.on_click.clone()}
        >
            <span>{props.children.clone()}</span>
        </button>
    }
}
