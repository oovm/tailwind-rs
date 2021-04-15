#![forbid(missing_debug_implementations)]
#![forbid(missing_crate_level_docs)]
#![doc = include_str ! ("../Readme.md")]

mod resolver;
mod systems;
mod units;
mod traits;
mod builder;

pub use self::{
    resolver::TailwindConfig,
    systems::{
        breakpoints::{BreakPoint, BreakPointSystem},
        colors::{Palette, PaletteSystem},
        fonts::FontSystem,
        preflight::PreflightSystem,
        sizes::{TailwindMaxWidth, TailwindMinWidth, TailwindWidth},
    },
    traits::CssAttribute
};


