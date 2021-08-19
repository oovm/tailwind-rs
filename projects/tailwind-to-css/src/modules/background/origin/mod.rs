use crate::{css_attributes, CssAttribute, CssBehavior, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

#[derive(Copy, Clone, Debug)]
enum BackgroundOrigin {
    Border,
    Padding,
    Content,
    Global(CssBehavior),
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundOrigin {
    kind: BackgroundOrigin,
}

impl Display for BackgroundOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Border => write!(f, "border"),
            Self::Padding => write!(f, "padding"),
            Self::Content => write!(f, "content"),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindBackgroundOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-origin-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBackgroundOrigin {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let clip = match &self.kind {
            BackgroundOrigin::Border => "border-box".to_string(),
            BackgroundOrigin::Padding => "padding-box".to_string(),
            BackgroundOrigin::Content => "content-box".to_string(),
            BackgroundOrigin::Global(g) => g.to_string(),
        };
        css_attributes! {
            "background-clip" => clip
        }
    }
}

impl TailwindBackgroundOrigin {
    /// `bg-clip-border`
    pub const Border: Self = Self { kind: BackgroundOrigin::Border };
    /// `bg-clip-padding`
    pub const Padding: Self = Self { kind: BackgroundOrigin::Padding };
    /// `bg-clip-content`
    pub const Content: Self = Self { kind: BackgroundOrigin::Content };
}
