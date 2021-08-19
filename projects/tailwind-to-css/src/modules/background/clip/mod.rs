use crate::{css_attributes, CssAttribute, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
};

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundClip {
    kind: BackgroundClip,
}

#[derive(Copy, Clone, Debug)]
enum BackgroundClip {
    Border,
    Padding,
    Content,
    Text,
    Global(CssAttribute),
}

impl Display for BackgroundClip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Border => write!(f, "border"),
            Self::Padding => write!(f, "padding"),
            Self::Content => write!(f, "content"),
            Self::Text => write!(f, "text"),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindBackgroundClip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-clip-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBackgroundClip {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let clip = match &self.kind {
            BackgroundClip::Border => "border-box".to_string(),
            BackgroundClip::Padding => "padding-box".to_string(),
            BackgroundClip::Content => "content-box".to_string(),
            BackgroundClip::Text => "text".to_string(),
            BackgroundClip::Global(g) => g.to_string(),
        };
        css_attributes! {
            "background-clip" => clip
        }
    }
}

impl TailwindBackgroundClip {
    /// `bg-clip-border`
    pub const Border: Self = Self { kind: BackgroundClip::Border };
    /// `bg-clip-padding`
    pub const Padding: Self = Self { kind: BackgroundClip::Padding };
    /// `bg-clip-content`
    pub const Content: Self = Self { kind: BackgroundClip::Content };
    /// `bg-clip-text`
    pub const Text: Self = Self { kind: BackgroundClip::Text };
}
