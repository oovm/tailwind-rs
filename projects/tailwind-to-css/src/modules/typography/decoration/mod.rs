pub use self::{color::TailwindDecorationColor, line::TailwindTextDecoration};
use super::*;
use crate::{modules::typography::decoration::style::TailwindDecorationStyle, TailwindColor};

mod color;
mod line;
mod style;
mod thickness;

// decoration-inherit	text-decoration-color: inherit;
// decoration-current	text-decoration-color: currentColor;
// decoration-transparent	text-decoration-color: transparent;
// decoration-black	text-decoration-color: #000;
// decoration-white	text-decoration-color: #fff;
// decoration-slate-50	text-decoration-color: #f8fafc;

// decoration-solid	text-decoration-style: solid;
// decoration-double	text-decoration-style: double;
// decoration-dotted	text-decoration-style: dotted;
// decoration-dashed	text-decoration-style: dashed;
// decoration-wavy	text-decoration-style: wavy;

// decoration-auto	text-decoration-thickness: auto;
// decoration-from-font	text-decoration-thickness: from-font;
// decoration-0	text-decoration-thickness: 0px;
// decoration-1	text-decoration-thickness: 1px;
// decoration-2	text-decoration-thickness: 2px;
// decoration-4	text-decoration-thickness: 4px;
// decoration-8	text-decoration-thickness: 8px;

#[inline]
pub(crate) fn decoration_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    debug_assert!(arbitrary.is_none(), "forbidden arbitrary after justify");
    let out = match str {
        ["solid", _rest @ ..] => TailwindDecorationStyle::Solid.boxed(),

        // https://tailwindcss.com/docs/justify-content
        ["solid", _rest @ ..] => TailwindListStyle::None.boxed(),
        // https://tailwindcss.com/docs/justify-items
        ["items", _rest @ ..] => TailwindListStyle::None.boxed(),
        // https://tailwindcss.com/docs/justify-self
        ["self", _rest @ ..] => TailwindListStyle::None.boxed(),
        _ => return syntax_error!("Unknown justify instructions: {}", str.join("-")),
    };
    Ok(out)
}

#[doc = include_str!("text-decoration-thickness.md")]
#[derive(Debug, Clone)]
pub enum TailwindDecorationThickness {
    /// <p style="text-decoration-line:underline;text-decoration-thickness:auto;">The quick brown fox jumps over the lazy dog.</p>
    Auto,
    /// <p style="text-decoration-line:underline;text-decoration-thickness:from-font;">The quick brown fox jumps over the lazy dog.</p>
    FromFont,
    /// <p style="text-decoration-line:underline;text-decoration-thickness:2px;">The quick brown fox jumps over the lazy dog.</p>
    Unit(usize),
}

impl TailwindInstance for TailwindDecorationThickness {}
