//! Candidate syntax tree produced by `tailwind-parser`.
//!
//! Syntax only — no pseudo/breakpoint semantics, no CSS strings.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use tailwind_types::TextRange;

/// One fully parsed candidate after group desugar.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateSyntax {
    pub variants: Vec<VariantSyntax>,
    pub important: bool,
    pub negative: bool,
    pub utility: UtilitySyntax,
    pub span: TextRange,
}

/// A single variant prefix. Semantics (hover/md/dark) are resolved later.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariantSyntax {
    pub not: bool,
    pub body: VariantBody,
    pub span: TextRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum VariantBody {
    /// `hover:` / `group-hover:` / `first-line::` — `double_colon` is lexical only.
    Named {
        names: Vec<Identifier>,
        double_colon: bool,
    },
    /// `[&:nth-child(3)]:` / `[@media(min-width:768px)]:`
    Arbitrary(ArbitrarySyntax),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum UtilitySyntax {
    /// `block`, `p-4`, `bg-red-500/50`, `p-[1.5rem]`
    Standard {
        name: Identifier,
        value: UtilityValue,
        modifier: Option<ModifierSyntax>,
    },
    /// `[margin:1px]` / `[--my-var:1px]`
    ArbitraryProperty {
        property: Identifier,
        value: ArbitrarySyntax,
        modifier: Option<ModifierSyntax>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum UtilityValue {
    Bare,
    Named(NamedValue),
    Arbitrary(ArbitrarySyntax),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedValue {
    /// Full text after the utility name dash, e.g. `red-500` or `1/2`.
    pub text: String,
    pub span: TextRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArbitrarySyntax {
    /// Optional type hint before `:`, e.g. `length` in `[length:var(--x)]`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_hint: Option<Identifier>,
    /// Inner text with escapes preserved as written (not re-escaped).
    pub raw: String,
    /// Span covering `[` … `]`.
    pub span: TextRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifierSyntax {
    pub value: ModifierValue,
    pub span: TextRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ModifierValue {
    Named(Identifier),
    Arbitrary(ArbitrarySyntax),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    pub text: String,
    pub span: TextRange,
}

impl Identifier {
    pub fn new(text: impl Into<String>, span: TextRange) -> Self {
        Self {
            text: text.into(),
            span,
        }
    }
}
