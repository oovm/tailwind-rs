mod display;
mod parser;
use super::*;

/// font that unknown at parsing time
#[derive(Debug, Clone)]
pub enum TailwindFontArbitrary {
    Unsolved,
}

#[doc = include_str!("font-family.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontFamily {
    name: String,
}

#[doc = include_str!("font-family.md")]
#[derive(Copy, Debug, Clone)]
pub struct TailwindFontSize {
    size: TailwindTracking,
    height: TailwindLeading,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontSmoothing {
    Normal,
    Subpixel,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindFontStyle {
    Italic,
    Normal,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub struct TailwindFontWeight {
    weight: usize,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Copy, Debug, Clone)]
pub enum TailwindFontVariantNumeric {
    Normal,
    Ordinal,
    SlashedZero,
    Lining,
    OldStyle,
    Proportional,
    Tabular,
    DiagonalFractions,
    StackedFractions,
}

#[doc = include_str!("letter-spacing.md")]
#[derive(Copy, Debug, Clone)]
pub enum TailwindTracking {
    Normal,
    Inherit,
    Initial,
    Unset,
    Em(f32),
}

#[doc = include_str!("line-height.md")]
#[derive(Copy, Debug, Clone)]
pub enum TailwindLeading {
    Normal,
    Inherit,
    Initial,
    Unset,
    Unit(usize),
    Scale(f32),
    Rem(f32),
    // Px(f32),
}

#[doc = include_str!("line-height.md")]
#[derive(Copy, Debug, Clone)]
pub enum TailwindListStyleType {
    None,
    Disc,
    Decimal,
}

#[doc = include_str!("line-height.md")]
#[derive(Copy, Debug, Clone)]
pub enum TailwindListStylePosition {
    Inside,
    Outside,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindTextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextColor {
    pub(crate) color: ColorResolver,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindTextDecoration {
    Underline,
    Overline,
    ThroughLine,
    None,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextDecorationColor {
    pub(crate) color: ColorResolver,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindTextDecorationStyle {
    /// <p style="text-decoration-line:underline;text-decoration-style:solid;">The quick brown fox jumps over the lazy dog.</p>
    Solid,
    /// <p style="text-decoration-line:underline;text-decoration-style:double;">The quick brown fox jumps over the lazy dog.</p>
    Double,
    /// <p style="text-decoration-line:underline;text-decoration-style:dotted;">The quick brown fox jumps over the lazy dog.</p>
    Dotted,
    /// <p style="text-decoration-line:underline;text-decoration-style:dashed;">The quick brown fox jumps over the lazy dog.</p>
    Dashed,
    /// <p style="text-decoration-line:underline;text-decoration-style:wavy;">The quick brown fox jumps over the lazy dog.</p>
    Wavy,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindTextDecorationThickness {
    /// <p style="text-decoration-line:underline;text-decoration-thickness:auto;">The quick brown fox jumps over the lazy dog.</p>
    Auto,
    /// <p style="text-decoration-line:underline;text-decoration-thickness:from-font;">The quick brown fox jumps over the lazy dog.</p>
    FromFont,
    /// <p style="text-decoration-line:underline;text-decoration-thickness:2px;">The quick brown fox jumps over the lazy dog.</p>
    Unit(usize),
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindTextUnderlineOffset {
    Auto,
    Unit(usize),
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindTextTransform {
    Uppercase,
    Lowercase,
    Capitalize,
    None,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindTextOverflow {
    Truncate,
    Ellipsis,
    Clip,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindTextIndent {
    Px(f32),
    Unit(f32),
    Percent(f32),
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindAlign {
    Baseline,
    Top,
    Middle,
    Bottom,
    TextTop,
    TextBottom,
    Sub,
    Super,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindWhiteSpace {
    Normal,
    NoWrap,
    Pre,
    PreLine,
    PreWrap,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindBreak {
    Normal,
    Words,
    All,
}

#[doc = include_str!("font-smoothing.md")]
#[derive(Debug, Clone)]
pub enum TailwindContent {
    None,
}
