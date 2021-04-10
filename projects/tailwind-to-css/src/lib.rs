#![forbid(missing_debug_implementations)]
#![forbid(missing_crate_level_docs)]
#![doc=include_str!("../Readme.md")]

mod systems;

pub use systems::breakpoints::{BreakPointSystem, BreakPoint};
pub use systems::colors::{PaletteSystem, Palette};