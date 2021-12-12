#![forbid(missing_docs)]
#![doc = include_str!("../readme.md")]
#![allow(clippy::needless_return)]

mod error;
mod error_3rd;

pub use self::error::{MaybeRanged, Result, TailwindError, TailwindErrorKind};
pub use url::Url;

#[cfg(feature = "git2")]
pub extern crate git2;
#[cfg(feature = "globset")]
pub extern crate globset;
#[cfg(feature = "nom")]
pub extern crate nom;
#[cfg(feature = "num")]
pub extern crate num;
