use super::*;
///
#[derive(Copy, Clone, Debug)]
pub struct TailwindOutlineStyle {
    kind: BorderStyle,
}

impl Display for TailwindOutlineStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            BorderStyle::Solid => write!(f, "outline"),
            _ => write!(f, "outline-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindOutlineStyle {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match self.kind {
            BorderStyle::None => css_attributes! {
                "outline" => "2px solid transparent",
                "outline-offset" => "2px",
            },
            _ => css_attributes! {
                "outline-style" => self.kind
            },
        }
    }
}

impl TailwindOutlineStyle {
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
