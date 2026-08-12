//! Resolve kernel: theme snapshot, typed values, rule/variant registries.
//!
//! Rules return structured [`DeclarationSet`] values — never CSS rule strings.

#![forbid(unsafe_code)]

mod builtin;
mod canonical;
mod context;
mod registry;
mod rules;
mod stable_hash;
mod theme;
mod typed;
mod variants;

pub use builtin::{
    builtin_theme_input, default_breakpoints, default_rule_registry, default_theme,
    default_variant_registry, register_all_families,
};
pub use canonical::{canonicalize, CanonicalizeOutput};
pub use context::{ResolveContext, ResolvedRule};
pub use registry::{RuleKind, RuleRegistry, RuleSpec};
pub use rules::generic::{
    KeywordMapRule, MinMaxKind, StaticRule, ThemeColorRule, ThemeLengthRule,
};
pub use rules::spacing::{SpacingAxis, SpacingKind, SpacingRule};
pub use stable_hash::{hash_str, StableHasher};
pub use theme::{ThemeLookup, ThemeSnapshot, ORDERING_VERSION};
pub use typed::{ColorResolver, KeywordResolver, LengthResolver, TypedValue};
pub use variants::{VariantEffect, VariantRegistry};

use tailwind_ast::CandidateSyntax;
use tailwind_types::{
    CanonicalRule, ConditionTree, DeclarationSet, Diagnostic, DiagnosticCode, OrderKey, Provenance,
    ThemeKey,
};

/// Resolve one parsed candidate against registries + theme.
pub fn resolve_candidate(
    syntax: &CandidateSyntax,
    candidate_key: &str,
    provenance: Provenance,
    rules: &RuleRegistry,
    variants: &VariantRegistry,
    theme: &ThemeSnapshot,
) -> Result<ResolvedRule, Diagnostic> {
    let mut ctx = ResolveContext::new(theme);
    let conditions = variants.resolve_all(&syntax.variants, &mut ctx)?;
    let (decls, utility_rank) = rules.resolve(syntax, &mut ctx)?;
    let mut declarations = decls.declarations;
    if syntax.important {
        for d in &mut declarations {
            d.important = true;
        }
    }

    let variant_rank = variants.rank_of(&syntax.variants);
    let order = OrderKey {
        layer: 3,
        variant_rank,
        utility_rank,
        tie: candidate_key.to_string(),
    };

    Ok(ResolvedRule {
        rule: CanonicalRule {
            candidate_key: candidate_key.to_string(),
            conditions,
            declarations,
            order,
            provenance: vec![provenance],
        },
        theme_reads: ctx.into_theme_reads(),
    })
}

/// Convenience: unknown utility diagnostic.
pub fn unknown_utility(name: &str) -> Diagnostic {
    Diagnostic::error(
        DiagnosticCode::ResolveUnknownUtility,
        format!("unknown utility `{name}`"),
    )
}

pub fn empty_declarations() -> DeclarationSet {
    DeclarationSet::default()
}

pub fn theme_key(parts: &[&str]) -> ThemeKey {
    ThemeKey::from_path(parts.iter().copied())
}

/// Ensure [`ConditionTree`] has a none default for empty variant lists.
pub fn condition_none() -> ConditionTree {
    ConditionTree::None
}
