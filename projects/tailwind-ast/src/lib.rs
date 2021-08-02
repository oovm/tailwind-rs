#![deny(missing_debug_implementations)]
#![deny(missing_crate_level_docs)]
#![deny(missing_copy_implementations)]
#![deny(missing_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg")]
#![doc(
    html_favicon_url = "https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg"
)]

mod ast;
mod utils;

pub use self::{ast::*, utils::*};
