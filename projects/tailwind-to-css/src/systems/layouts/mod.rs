use super::*;

pub mod builder;
pub mod parser;

/// https://tailwindcss.com/docs/aspect-ratio
pub struct LayoutSystem {}

/// https://tailwindcss.com/docs/aspect-ratio
///
/// ```
/// use tailwind_css::TailwindAspect;
///
/// #[test]
/// fn build_aspect() {
///     let mut out = String::new();
///     TailwindAspect::new("square", "1/1").write_css(&mut out);
///     assert_eq!(out, ".break-before: square; {\n    break-before: 1/1;\n}\n")
/// }
/// ```
#[derive(Copy, Clone, Debug)]
pub struct TailwindAspect {
    kind: &'static str,
    ratio: &'static str,
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

#[doc = include_str!("z-index.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailWindZIndex {
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

/// https://tailwindcss.com/docs/display
pub enum TailwindDisplay {
    Block,
    Inline,
    InlineBlock,
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
