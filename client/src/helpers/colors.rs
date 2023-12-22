#![allow(dead_code)]

use std::fmt::Display;

use yew::{classes, Classes};
#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Dark,
    Light,
    Hint,
}

impl Color {
    pub fn bg_color(&self) -> Classes {
        classes!(match self {
            Self::Primary => "bg-primary",
            Self::Secondary => "bg-secondary",
            Self::Success => "bg-success",
            Self::Warning => "bg-warning",
            Self::Danger => "bg-danger",
            Self::Dark => "bg-dark",
            Self::Light => "bg-light",
            _ => "bg-light",
        })
    }

    pub fn bg_hover(&self) -> Classes {
        classes!(match self {
            Self::Primary => "hover:bg-primary-light",
            Self::Secondary => "hover:bg-secondary-light",
            Self::Success => "hover:bg-success-light",
            Self::Warning => "hover:bg-warning-light",
            Self::Danger => "hover:bg-danger-light",
            Self::Dark => "hover:bg-grey",
            Self::Light => "hover:bg-grey",
            _ => "hover:bg-grey",
        })
    }
    pub fn text_color(&self) -> Classes {
        classes!(match self {
            Self::Primary => "text-primary",
            Self::Secondary => "text-secondary",
            Self::Success => "text-success",
            Self::Warning => "text-warning",
            Self::Danger => "text-danger",
            Self::Dark => "text-light",
            Self::Light => "text-dark",
            _ => "text-light",
        })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Dark => "dark",
            Self::Light => "light",
            Self::Hint => "hint",
        };

        write!(f, "{color}")
    }
}
