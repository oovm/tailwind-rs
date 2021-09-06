use super::*;
use crate::css_attributes;

///
#[derive(Copy, Clone, Debug)]
pub struct TailwindBorderStyle {
    kind: BorderStyle,
}

impl Display for TailwindBorderStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "border-{}", self.kind)
    }
}

impl TailwindInstance for TailwindBorderStyle {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "border-style" => self.kind
        }
    }
}

impl TailwindBorderStyle {
    /// `tracking-solid`
    pub const Solid: Self = Self { kind: BorderStyle::Solid };
    /// `tracking-dashed`
    pub const Dashed: Self = Self { kind: BorderStyle::Dashed };
    /// `tracking-dotted`
    pub const Dotted: Self = Self { kind: BorderStyle::Dotted };
    /// `tracking-double`
    pub const Double: Self = Self { kind: BorderStyle::Double };
    /// `tracking-hidden`
    pub const Hidden: Self = Self { kind: BorderStyle::Hidden };
    /// `tracking-none`
    pub const None: Self = Self { kind: BorderStyle::None };
}
