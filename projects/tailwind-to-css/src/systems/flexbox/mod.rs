mod builder;
mod display;
use super::*;

/// https://tailwindcss.com/docs/flex-basis
pub enum TailwindFlexBasis {
    Auto,
    Full,
}

/// https://tailwindcss.com/docs/flex-basis
pub enum TailwindFlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

pub struct TailwindFlexWrap {}

pub struct TailwindFlex {}

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
