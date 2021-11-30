pub use config::GlobalConfig;
pub use tailwind_error::{Result, TailwindError};

pub use self::processor::CssProcessor;

mod config;
mod processor;
mod support;
