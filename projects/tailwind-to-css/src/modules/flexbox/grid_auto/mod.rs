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
