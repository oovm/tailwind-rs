pub use self::config::CLIConfig;
pub use tailwind_error::{Result, TailwindError};

mod config;
mod processor;
mod support;

pub use tailwind_css::{CssInlineMode, TailwindBuilder};
