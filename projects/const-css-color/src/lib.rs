#![feature(const_for)]
#![feature(const_mut_refs)]
#![forbid(missing_docs)]
#![forbid(missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_doc_code_examples)]
#![doc = include_str!("../readme.md")]

mod color;

pub use self::color::Color;
