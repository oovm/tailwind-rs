use crate::TailwindInstance;
use std::fmt::{Display, Formatter};

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

impl Display for TailwindTextDecoration {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindTextDecoration {}

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
