#![forbid(missing_docs)]
#![doc = include_str!("../readme.md")]
#![allow(clippy::needless_return)]

mod error;
/// Third party error handler
pub mod error_3rd;

pub use self::error::{Result, TailwindError, TailwindErrorKind};
pub use diagnostic::TextStorage;
