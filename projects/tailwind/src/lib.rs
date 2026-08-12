//! User-facing Tailwind engine facade.
//!
//! ```ignore
//! use tailwind::*;
//! ```
//!
//! Crate layout:
//! - `tailwind-types` — compile contract + diagnostics (miette)
//! - `tailwind-ast` — candidate syntax tree
//! - `tailwind-parser` — built-in lexer + parser (no nom)
//! - `tailwind-resolve` — ThemeSnapshot, RuleRegistry, VariantRegistry, canonicalize
//! - `tailwind` — this facade (`Engine::compile`, re-exports)
//!
//! Pipeline: Parse → Resolve → Canonicalize → Canonical Style Module.
//! C5: layered caches (parse / resolve / module) + parallel candidate resolve.

#![forbid(unsafe_code)]

mod cache;
mod engine;

pub use engine::{compile_no_panic, is_internal_panic, Engine};
pub use tailwind_ast::*;
pub use tailwind_parser::{
    parse_candidate, parse_source, parse_token, Lexer, ParseError, ParseOutput, ParseResult, Token,
    TokenKind,
};
pub use tailwind_resolve::{
    canonicalize, default_rule_registry, default_theme, default_variant_registry,
    register_all_families, resolve_candidate, CanonicalizeOutput, ColorResolver, KeywordMapRule,
    LengthResolver, MinMaxKind, ResolveContext, ResolvedRule, RuleRegistry, RuleSpec, StableHasher,
    StaticRule, ThemeColorRule, ThemeLengthRule, ThemeLookup, ThemeSnapshot, TypedValue,
    VariantEffect, VariantRegistry, ORDERING_VERSION,
};
pub use tailwind_types::*;
