mod builder;
mod display;
use super::*;

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlexBasis {
    Auto,
    Full,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlexWrap {
    Wrap,
    WrapReverse,
    NoWrap,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlex {
    None,
    Inherit,
    Auto { grow: usize, shrink: usize },
    Percent { grow: usize, shrink: usize, basis: usize },
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindFlexGrow {
    grow: usize,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindFlexShrink {
    shrink: usize,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailWindOrder {
    order: usize,
    negative: bool,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindGap {
    Row,
    Column,
    Both,
}

#[doc=include_str!("justify-content.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindJustifyContent {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

#[doc=include_str!("justify-items.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindJustifyItems {
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindJustifySelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindAlignContent {
    // Auto,
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindAlignItems {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindAlignSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPlaceContent {
    // Auto,
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPlaceItems {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

#[doc=include_str!("justify-self.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindPlaceSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}
