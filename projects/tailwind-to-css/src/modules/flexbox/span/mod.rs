use super::*;

use self::resolve::GridKind;

mod resolve;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindRow {
    kind: GridKind,
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindColumn {
    kind: GridKind,
}

impl Display for TailwindRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "row-{}", self.kind)
    }
}

impl Display for TailwindColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "col-{}", self.kind)
    }
}

impl TailwindInstance for TailwindColumn {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = match self.kind {
            GridKind::Start(_) => "grid-column-start",
            GridKind::End(_) => "grid-column-end",
            GridKind::Span(_) => "grid-column",
        };
        css_attributes! {
            class => self.kind.get_properties()
        }
    }
}

impl TailwindInstance for TailwindRow {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = match self.kind {
            GridKind::Start(_) => "grid-row-start",
            GridKind::End(_) => "grid-row-end",
            GridKind::Span(_) => "grid-row",
        };
        css_attributes! {
            class => self.kind.get_properties()
        }
    }
}

impl TailwindRow {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: GridKind::parse(pattern, arbitrary)? })
    }
}

impl TailwindColumn {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: GridKind::parse(pattern, arbitrary)? })
    }
}
