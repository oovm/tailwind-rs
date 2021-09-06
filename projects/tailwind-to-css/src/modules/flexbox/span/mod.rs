use self::resolve::GridKind;
use super::*;
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
        todo!()
    }
}

impl Display for TailwindColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindColumn {}

impl TailwindInstance for TailwindRow {}

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
