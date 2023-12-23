use gloo::console::log;
use yew::{classes, function_component, html, virtual_dom::VNode, AttrValue, Classes, Properties};

use crate::helpers::{
    colors::Color,
    flex::{FlexAlign, FlexDirection, FlexGap, FlexJustify},
};

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub children: VNode,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub bg: Option<Color>,
}

#[function_component(Container)]
pub fn component(props: &ContainerProps) -> VNode {
    let classes = classes!(
        props.classes.clone(),
        match props.bg {
            Some(c) => c.bg_color(),
            None => classes!("bg-inherit"),
        }
    );

    html! {
        <div class={classes}>{props.children.clone()}</div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FlexContainerProps {
    pub children: VNode,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub bg: Option<Color>,
    #[prop_or_default]
    pub row: bool,
    #[prop_or(FlexJustify::Start)]
    pub justify: FlexJustify,
    #[prop_or(FlexAlign::Stretch)]
    pub align: FlexAlign,
    #[prop_or_default]
    pub gap: Option<FlexGap>,
}

#[function_component(FlexContainer)]
pub fn component(props: &FlexContainerProps) -> VNode {
    let classes = classes!(
        ["flex"],
        props.classes.clone(),
        props.justify.class(),
        props.align.class(),
        match props.row {
            true => FlexDirection::Row.class(),
            _ => FlexDirection::Col.class(),
        },
        match props.gap {
            Some(g) => g.class(),
            None => classes!("gap-0"),
        },
        match props.bg {
            Some(c) => c.bg_color(),
            None => classes!("bg-inherit"),
        }
    );

    log!(format!("{classes:?}"));

    html! {
        <div class={classes}>{props.children.clone()}</div>
    }
}
