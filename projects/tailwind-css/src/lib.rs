//! Reference CSS serializer for Canonical Style Module
//! ([`tailwind_types::CanonicalStyleModule`]).
//!
//! This crate is **not** part of the engine core. `Engine::compile` stops at the
//! structured module; hosts call [`serialize_module`] (or VMZ/Doki adapters) to
//! lower into CSS or other surfaces.
//!
//! Do not put `Display` / stylesheet strings back into `tailwind` / `tailwind-resolve`.

#![forbid(unsafe_code)]

mod escape;
mod serialize;

pub use escape::escape_class_name;
pub use serialize::{serialize_module, serialize_module_with, serialize_rule, SerializeOptions};
