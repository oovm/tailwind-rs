mod display;
mod parser;
use super::*;

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindAccentColor {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindAppearance {
    None,
}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindCursor {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindCaretColor {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPointerEvents {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindResize {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindScroll {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindSnap {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindTorch {
    Auto,
    None,
    PanX,
    PanL,
    PanR,
    PanY,
    PanU,
    PanD,
    PinchZoom,
    Manipulate,
}

#[derive(Debug, Copy, Clone)]
enum SelectKind {
    None,
    Auto,
    Text,
    Contain,
    All,
    Global(CssBehavior),
}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindSelect {
    kind: SelectKind,
}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindWillChange {}
