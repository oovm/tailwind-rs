use super::*;

#[doc = include_str!("text-decoration.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindTextDecoration {
    kind: TextDecoration,
}

#[derive(Debug, Copy, Clone)]
enum TextDecoration {
    Underline,
    Overline,
    ThroughLine,
    None,
}

impl Display for TextDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Underline => write!(f, "underline"),
            Self::Overline => write!(f, "overline"),
            Self::ThroughLine => write!(f, "line-through"),
            Self::None => write!(f, "underline-none"),
        }
    }
}

impl Display for TailwindTextDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl TailwindInstance for TailwindTextDecoration {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let line = match self.kind {
            TextDecoration::Underline => "underline",
            TextDecoration::Overline => "overline",
            TextDecoration::ThroughLine => "line-through",
            TextDecoration::None => "none",
        };
        css_attributes! {
            "text-decoration-line" => line
        }
    }
}

impl TailwindTextDecoration {
    /// `underline`
    pub const Underline: Self = Self { kind: TextDecoration::Underline };
    /// `overline`
    pub const Overline: Self = Self { kind: TextDecoration::Overline };
    /// `line-through`
    pub const ThroughLine: Self = Self { kind: TextDecoration::ThroughLine };
    /// `no-underline`
    pub const None: Self = Self { kind: TextDecoration::None };
}
