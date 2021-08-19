use crate::{css_attributes, CssAttribute, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

#[derive(Copy, Clone, Debug)]
enum BackgroundRepeat {
    Repeat,
    RepeatX,
    RepeatY,
    Round,
    Space,
    None,
    Global(CssAttribute),
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundRepeat {
    kind: BackgroundRepeat,
}

impl Display for BackgroundRepeat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Repeat => write!(f, "repeat"),
            Self::RepeatX => write!(f, "repeat-x"),
            Self::RepeatY => write!(f, "repeat-y"),
            Self::Round => write!(f, "repeat-round"),
            Self::Space => write!(f, "repeat-space"),
            Self::None => write!(f, "no-repeat"),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindBackgroundRepeat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBackgroundRepeat {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let clip = match &self.kind {
            BackgroundRepeat::Repeat => "repeat".to_string(),
            BackgroundRepeat::RepeatX => "repeat-x".to_string(),
            BackgroundRepeat::RepeatY => "repeat-y".to_string(),
            BackgroundRepeat::Round => "round".to_string(),
            BackgroundRepeat::Space => "space".to_string(),
            BackgroundRepeat::None => "no-repeat".to_string(),
            BackgroundRepeat::Global(g) => g.to_string(),
        };
        css_attributes! {
            "background-repeat" => clip
        }
    }
}

impl TailwindBackgroundRepeat {
    /// `bg-repeat`
    pub const Repeat: Self = Self { kind: BackgroundRepeat::Repeat };
    /// `bg-no-repeat`
    pub const RepeatX: Self = Self { kind: BackgroundRepeat::RepeatX };
    /// `bg-repeat-x`
    pub const RepeatY: Self = Self { kind: BackgroundRepeat::RepeatY };
    /// `bg-repeat-y`
    pub const Round: Self = Self { kind: BackgroundRepeat::Round };
    /// `bg-repeat-round`
    pub const Space: Self = Self { kind: BackgroundRepeat::Space };
    /// `bg-repeat-space`
    pub const None: Self = Self { kind: BackgroundRepeat::None };
}
