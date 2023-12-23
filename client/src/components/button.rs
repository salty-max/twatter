use web_sys::MouseEvent;
use yew::{classes, function_component, html, virtual_dom::VNode, Callback, Classes, Properties};
use yew_icons::{Icon, IconId};

use crate::helpers::colors::Color;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: VNode,
    pub variant: Color,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub icon: Option<IconId>,
}

#[function_component(Button)]
pub fn component(props: &ButtonProps) -> VNode {
    let classes = classes!(
        [
            "flex",
            "px-4",
            "py-2",
            "rounded-md",
            "transition",
            "ease-in-out",
            "delay-150",
            "duration-300"
        ],
        props.variant.bg_color(),
        props.classes.clone(),
        props.icon.map(|_| "gap-2").unwrap_or(""),
        match props.disabled {
            true => classes!("opacity-70", "cursor-not-allowed"),
            _ => classes!("hover:scale-110", "hover:-translate-y-1",),
        },
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
            {
                props.icon.map(|icon| {
                    html! {
                        <Icon icon_id={icon} width={"1rem"} />
                    }
                })
            }
            <span>{props.children.clone()}</span>
        </button>
    }
}
