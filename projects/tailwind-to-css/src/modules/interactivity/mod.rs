use crate::TailwindSizing;

use super::*;

pub use self::{
    cursor::TailwindCursor, pointer::TailwindPointerEvents, resize::TailwindResize, select::TailwindSelect,
    torch::TailwindTorch,
};

mod cursor;
mod display;
mod parser;
mod pointer;
mod resize;
mod select;
mod torch;

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
pub enum TailwindCaretColor {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindScrollSize {
    wrapper: TailwindSizing,
}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindScroll {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindSnap {}

#[doc=include_str!("grid-row.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindWillChange {}
