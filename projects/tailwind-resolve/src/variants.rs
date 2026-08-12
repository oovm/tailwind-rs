use tailwind_ast::{VariantBody, VariantSyntax};
use tailwind_types::{
    AtRuleCondition, ConditionTree, Diagnostic, DiagnosticCode, SelectorCondition,
};

use crate::context::ResolveContext;

/// Effect produced by a resolved variant (no CSS string assembly in rules).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VariantEffect {
    SelectorTransform { transform: String },
    AtRule { name: String, query: String },
}

#[derive(Clone, Debug)]
struct VariantDef {
    effect: VariantEffect,
    order: u32,
    allows_not: bool,
}

/// Named variant registry. Arbitrary variants are handled structurally.
#[derive(Clone, Debug, Default)]
pub struct VariantRegistry {
    named: std::collections::BTreeMap<String, VariantDef>,
}

impl VariantRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_selector(
        &mut self,
        name: impl Into<String>,
        transform: impl Into<String>,
        order: u32,
        allows_not: bool,
    ) {
        self.named.insert(
            name.into(),
            VariantDef {
                effect: VariantEffect::SelectorTransform {
                    transform: transform.into(),
                },
                order,
                allows_not,
            },
        );
    }

    pub fn register_at_rule(
        &mut self,
        name: impl Into<String>,
        at_name: impl Into<String>,
        query: impl Into<String>,
        order: u32,
    ) {
        self.named.insert(
            name.into(),
            VariantDef {
                effect: VariantEffect::AtRule {
                    name: at_name.into(),
                    query: query.into(),
                },
                order,
                allows_not: false,
            },
        );
    }

    pub fn resolve_all(
        &self,
        variants: &[VariantSyntax],
        ctx: &mut ResolveContext<'_>,
    ) -> Result<ConditionTree, Diagnostic> {
        if variants.is_empty() {
            return Ok(ConditionTree::None);
        }

        let mut parts = Vec::with_capacity(variants.len());
        for v in variants {
            parts.push(self.resolve_one(v, ctx)?);
        }

        if parts.len() == 1 {
            Ok(parts.pop().unwrap())
        } else {
            Ok(ConditionTree::Compound(parts))
        }
    }

    pub fn rank_of(&self, variants: &[VariantSyntax]) -> u32 {
        let mut rank = 0u32;
        for v in variants {
            let name = variant_lookup_name(v);
            if let Some(def) = self.named.get(&name) {
                rank = rank.saturating_add(def.order);
            } else if ctx_is_breakpoint_name(&name) {
                rank = rank.saturating_add(20);
            } else {
                rank = rank.saturating_add(10_000);
            }
        }
        rank
    }

    /// Stable registry fingerprint for resolve_cache keys (C5).
    pub fn content_version(&self) -> u64 {
        use crate::stable_hash::StableHasher;
        let mut h = StableHasher::new();
        h.write_str("tw-variants-v1");
        for (name, def) in &self.named {
            h.write_str(name);
            h.write_u32(def.order);
            h.write_bool(def.allows_not);
            match &def.effect {
                VariantEffect::SelectorTransform { transform } => {
                    h.write_u8(1);
                    h.write_str(transform);
                }
                VariantEffect::AtRule { name, query } => {
                    h.write_u8(2);
                    h.write_str(name);
                    h.write_str(query);
                }
            }
        }
        h.finish()
    }

    fn resolve_one(
        &self,
        variant: &VariantSyntax,
        ctx: &mut ResolveContext<'_>,
    ) -> Result<ConditionTree, Diagnostic> {
        match &variant.body {
            VariantBody::Named { names, double_colon } => {
                let key = names
                    .iter()
                    .map(|n| n.text.as_str())
                    .collect::<Vec<_>>()
                    .join("-");

                if let Some(query) = ctx.lookup_breakpoint(&key) {
                    if variant.not {
                        return Err(Diagnostic::error(
                            DiagnosticCode::VariantNotCombinable,
                            format!("`not-` is not allowed on breakpoint `{key}`"),
                        ));
                    }
                    return Ok(ConditionTree::AtRule(AtRuleCondition {
                        name: "media".into(),
                        query: query.to_string(),
                    }));
                }

                let Some(def) = self.named.get(&key) else {
                    return Err(Diagnostic::error(
                        DiagnosticCode::VariantUnknown,
                        format!("unknown variant `{key}`"),
                    ));
                };

                if variant.not && !def.allows_not {
                    return Err(Diagnostic::error(
                        DiagnosticCode::VariantNotCombinable,
                        format!("`not-` is not allowed on variant `{key}`"),
                    ));
                }

                Ok(match &def.effect {
                    VariantEffect::SelectorTransform { transform } => {
                        let mut t = transform.clone();
                        if *double_colon && !t.starts_with("::") {
                            if let Some(rest) = t.strip_prefix(':') {
                                t = format!("::{rest}");
                            }
                        }
                        if variant.not {
                            t = format!(":not({t})");
                        }
                        ConditionTree::Selector(SelectorCondition { transform: t })
                    }
                    VariantEffect::AtRule { name, query } => {
                        ConditionTree::AtRule(AtRuleCondition {
                            name: name.clone(),
                            query: query.clone(),
                        })
                    }
                })
            }
            VariantBody::Arbitrary(a) => {
                if variant.not {
                    return Err(Diagnostic::error(
                        DiagnosticCode::VariantNotCombinable,
                        "arbitrary variants do not support `not-` in C2",
                    ));
                }
                let raw = if let Some(hint) = &a.type_hint {
                    format!("{}:{}", hint.text, a.raw)
                } else {
                    a.raw.clone()
                };
                if let Some(query) = raw.strip_prefix('@') {
                    Ok(ConditionTree::AtRule(AtRuleCondition {
                        name: "media".into(),
                        query: query.to_string(),
                    }))
                } else {
                    Ok(ConditionTree::Selector(SelectorCondition {
                        transform: raw,
                    }))
                }
            }
        }
    }
}

fn variant_lookup_name(v: &VariantSyntax) -> String {
    match &v.body {
        VariantBody::Named { names, .. } => names
            .iter()
            .map(|n| n.text.as_str())
            .collect::<Vec<_>>()
            .join("-"),
        VariantBody::Arbitrary(_) => "arbitrary".into(),
    }
}

fn ctx_is_breakpoint_name(name: &str) -> bool {
    matches!(name, "sm" | "md" | "lg" | "xl" | "2xl")
}
