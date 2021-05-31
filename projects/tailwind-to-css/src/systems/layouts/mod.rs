use super::*;

pub mod builder;
pub mod parser;

/// https://tailwindcss.com/docs/aspect-ratio
pub struct LayoutSystem {}

#[doc = include_str ! ("aspect-ratio.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindAspect {
    kind: &'static str,
    ratio: &'static str,
}

/// https://tailwindcss.com/docs/display
pub enum TailwindDisplay {
    Block,
    Inline,
    InlineBlock,
    Flex,
    InlineFlex,
    Table,
    InlineTable,
    TableCaption,
}

#[derive(Copy, Clone, Debug)]
pub enum TailwindBreak {
    /// https://tailwindcss.com/docs/break-before
    Before(&'static str),
    /// https://tailwindcss.com/docs/break-after
    After(&'static str),
    /// https://tailwindcss.com/docs/break-inside
    Inside(&'static str),
}

#[doc = include_str ! ("z-index.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailWindZIndex {
    Auto,
    Positive(usize),
    Negative(usize),
}

#[doc = include_str ! ("box-sizing.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailWindBoxSizing {
    Auto,
    Positive(isize),
    Negative(isize),
}

/// https://tailwindcss.com/docs/container
pub struct TailwindContainer {}

/// https://tailwindcss.com/docs/columns
pub struct TailwindColumns {}

/// https://tailwindcss.com/docs/box-sizing
pub enum TailwindBoxSizing {
    Border,
    Content,
}

/// https://tailwindcss.com/docs/float
pub enum TailwindFloat {
    Left,
    Right,
    None,
}

/// https://tailwindcss.com/docs/clear
pub enum TailwindClear {
    Left,
    Right,
    Both,
    None,
}

pub enum TailwindIsolation {
    Isolate,
    Auto,
}

/// https://tailwindcss.com/docs/object-fit
pub enum TailwindObjectFit {}

/// https://tailwindcss.com/docs/overscroll-behavior
pub enum TailwindOverscroll {}

/// https://tailwindcss.com/docs/position
pub enum TailwindPosition {}

/// https://tailwindcss.com/docs/visibility
pub enum TailwindVisibility {
    Visible,
    Invisible,
}
