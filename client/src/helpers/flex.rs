#![allow(dead_code)]

use yew::{classes, Classes};

#[derive(PartialEq, Copy, Clone)]
pub enum FlexDirection {
    Row,
    Col,
}

impl FlexDirection {
    pub fn class(&self) -> Classes {
        let class = match self {
            Self::Row => "flex-row",
            Self::Col => "flex-col",
        };

        classes!(class)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum FlexJustify {
    Start,
    End,
    Center,
    Between,
    Evenly,
    Stretch,
}

impl FlexJustify {
    pub fn class(&self) -> Classes {
        let class = match self {
            Self::Start => "justify-start",
            Self::End => "justify-end",
            Self::Center => "justify-center",
            Self::Between => "justify-between",
            Self::Evenly => "justify-evenly",
            Self::Stretch => "justify-stretch",
        };

        classes!(class)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum FlexAlign {
    Start,
    End,
    Center,
    Between,
    Evenly,
    Stretch,
}

impl FlexAlign {
    pub fn class(&self) -> Classes {
        let class = match self {
            Self::Start => "items-start",
            Self::End => "items-end",
            Self::Center => "items-center",
            Self::Between => "items-between",
            Self::Evenly => "items-evenly",
            Self::Stretch => "items-stretch",
        };

        classes!(class)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum FlexGap {
    SM,
    MD,
    LG,
    XL,
}

impl FlexGap {
    pub fn class(&self) -> Classes {
        let class = match self {
            FlexGap::SM => "gap-2",
            FlexGap::MD => "gap-4",
            FlexGap::LG => "gap-8",
            FlexGap::XL => "gap-12",
        };

        classes!(class)
    }
}
