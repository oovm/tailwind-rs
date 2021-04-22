#![forbid(missing_debug_implementations)]
#![forbid(missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]

mod builder;
mod systems;
mod traits;
mod units;

pub use self::{
    builder::TailwindBuilder,
    systems::{
        breakpoints::{BreakPoint, BreakPointSystem},
        colors::{Palette, PaletteSystem},
        fonts::FontSystem,
        preflight::PreflightSystem,
        sizes::{TailwindMaxWidth, TailwindMinWidth, TailwindWidth},
    },
    traits::CssInstance,
};
pub use tailwind_error::{Result, TailwindError, TailwindErrorKind};
