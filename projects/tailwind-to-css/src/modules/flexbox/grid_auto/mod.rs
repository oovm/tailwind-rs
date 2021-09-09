use super::*;

#[derive(Debug, Copy, Clone)]
enum GridAutoKind {
    Auto,
    Min,
    Max,
    Fr,
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGridAuto {
    kind: GridAutoKind,
    layout: bool,
}

impl Display for TailwindGridAuto {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindGridAuto {}

impl TailwindGridAuto {
    pub fn parse(_pattern: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
}
