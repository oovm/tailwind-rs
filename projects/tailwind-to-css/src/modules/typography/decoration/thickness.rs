// decoration-0	text-decoration-thickness: 0px;
// decoration-1	text-decoration-thickness: 1px;
// decoration-2	text-decoration-thickness: 2px;
// decoration-4	text-decoration-thickness: 4px;
// decoration-8	text-decoration-thickness: 8px;

use super::*;

#[derive(Debug, Clone)]
enum Thickness {
    Auto,
    FromFont,
    Length(LengthUnit),
}

#[doc = include_str!("text-decoration-thickness.md")]
#[derive(Debug, Clone)]
pub struct TailwindDecorationThickness {
    kind: Thickness,
}

impl Display for TailwindDecorationThickness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDecorationThickness {}

impl TailwindDecorationThickness {
    /// `decoration-auto`
    pub const Auto: Self = Self { kind: Thickness::Auto };
    /// `decoration-from-font`
    pub const FromFont: Self = Self { kind: Thickness::FromFont };
}
