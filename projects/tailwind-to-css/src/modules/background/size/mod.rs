use crate::{css_attributes, CssAttribute, CssBehavior, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

#[derive(Copy, Clone, Debug)]
enum BackgroundSize {
    Auto,
    Cover,
    Contain,
    Global(CssBehavior),
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundSize {
    kind: BackgroundSize,
}

impl Display for BackgroundSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Cover => write!(f, "cover"),
            Self::Contain => write!(f, "contain"),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindBackgroundSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBackgroundSize {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "background-repeat" => self.kind
        }
    }
}

impl TailwindBackgroundSize {
    /// `bg-auto`
    pub const Repeat: Self = Self { kind: BackgroundSize::Auto };
    /// `bg-cover`
    pub const RepeatX: Self = Self { kind: BackgroundSize::Cover };
    /// `bg-contain`
    pub const RepeatY: Self = Self { kind: BackgroundSize::Contain };
}
