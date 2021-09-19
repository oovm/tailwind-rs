use super::*;

#[derive(Copy, Clone, Debug)]
enum TableLayout {
    Auto,
    Fixed,
    Global(CssBehavior),
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindTableLayout {
    kind: TableLayout,
}

impl Display for TableLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Fixed => write!(f, "fixed"),
            Self::Global(g) => write!(f, "{}", g),
        }
    }
}

impl Display for TailwindTableLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "table-{}", self.kind)
    }
}

impl TailwindInstance for TailwindTableLayout {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "table-layout" => self.kind
        }
    }
}

impl TailwindTableLayout {
    /// `table-auto`
    pub const Auto: Self = Self { kind: TableLayout::Auto };
    /// `table-fixed`
    pub const Fixed: Self = Self { kind: TableLayout::Fixed };
}
