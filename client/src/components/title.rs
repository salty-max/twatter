use std::fmt::Display;

use yew::{classes, function_component, html, virtual_dom::VNode, Classes, Properties};

use crate::helpers::colors::Color;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    #[prop_or(Color::Light)]
    pub variant: Color,
    pub level: TitleLevel,
    pub children: VNode,
    #[prop_or_default]
    pub center: bool,
    #[prop_or_default]
    pub bold: bool,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Title)]
pub fn component(props: &TitleProps) -> VNode {
    let variant = props.variant;
    let classes = classes!(
        props.classes.clone(),
        variant.text_color(),
        props.center.then_some("text-center"),
        props.bold.then_some("font-semibold"),
        match props.level {
            TitleLevel::One => "text-3xl",
            TitleLevel::Two => "text-2xl",
            _ => "text-xl",
        }
    );

    html! {
        <@{format!("h{}", props.level.clone())} class={classes}>{props.children.clone()}</@>
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub enum TitleLevel {
    One = 1,
    Two,
    Three,
}

impl Display for TitleLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
        }
    }
}
