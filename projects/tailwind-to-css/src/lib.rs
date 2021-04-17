#![forbid(missing_debug_implementations)]
#![forbid(missing_crate_level_docs)]
#![doc = include_str ! ("../Readme.md")]

mod builder;
mod resolver;
mod systems;
mod traits;
mod units;

pub use self::{
    resolver::TailwindConfig,
    systems::{
        breakpoints::{BreakPoint, BreakPointSystem},
        colors::{Palette, PaletteSystem},
        fonts::FontSystem,
        preflight::PreflightSystem,
        sizes::{TailwindMaxWidth, TailwindMinWidth, TailwindWidth},
    },
    traits::CssDisplay,
};
pub use tailwind_error::{Result, TailwindError, TailwindErrorKind};
