#![forbid(missing_debug_implementations)]
#![forbid(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg")]
#![doc(html_favicon_url = "https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg")]

pub use tailwind_error::{Result, TailwindError, TailwindErrorKind};

pub use tailwind_ast::{TailwindArbitrary, TailwindElements, TailwindVariant};

pub use self::{modules::*, systems::*, traits::TailwindInstance};

mod macros;
mod modules;
mod systems;
mod traits;
