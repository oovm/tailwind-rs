//! Canonicalize resolved rules into a deterministic [`CanonicalStyleModule`].
//!
//! - normalize ConditionTree / declaration order
//! - merge by conditions: property-level last-wins with conflict diagnostics
//! - dedupe identical merged rules, merging provenance
//! - stable content hash (FNV-1a, process-independent)

use std::collections::BTreeMap;

use tailwind_types::{
    CanonicalRule, CanonicalStyleModule, ConditionTree, Declaration, Diagnostic, DiagnosticCode,
    OrderKey, Provenance, ThemeKey,
};

use crate::stable_hash::StableHasher;

/// Result of canonicalization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalizeOutput {
    pub module: CanonicalStyleModule,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Debug)]
struct PropEntry {
    decl: Declaration,
    order: OrderKey,
    candidate_key: String,
    provenance: Vec<Provenance>,
}

/// Canonicalize resolved rules + theme read-set into a stable module.
pub fn canonicalize(
    rules: Vec<CanonicalRule>,
    referenced_theme_keys: Vec<ThemeKey>,
) -> CanonicalizeOutput {
    let mut diagnostics = Vec::new();

    // condition -> property -> winning entry
    let mut buckets: BTreeMap<ConditionTree, BTreeMap<String, PropEntry>> = BTreeMap::new();

    for rule in rules.into_iter().map(normalize_rule) {
        let bucket = buckets.entry(rule.conditions.clone()).or_default();
        for decl in rule.declarations {
            let property = decl.property.clone();
            match bucket.get(&property) {
                None => {
                    bucket.insert(
                        property,
                        PropEntry {
                            decl,
                            order: rule.order.clone(),
                            candidate_key: rule.candidate_key.clone(),
                            provenance: rule.provenance.clone(),
                        },
                    );
                }
                Some(prev) => {
                    let same =
                        prev.decl.value == decl.value && prev.decl.important == decl.important;
                    let replace = rule.order > prev.order
                        || (rule.order == prev.order && rule.candidate_key > prev.candidate_key);

                    if !same {
                        let (kept, dropped) = if replace {
                            (preview_value(&decl.value), preview_value(&prev.decl.value))
                        } else {
                            (preview_value(&prev.decl.value), preview_value(&decl.value))
                        };
                        diagnostics.push(
                            Diagnostic::error(
                                DiagnosticCode::CanonicalConflict,
                                format!(
                                    "conflicting `{}` under the same conditions: dropped `{dropped}`, kept `{kept}`",
                                    property
                                ),
                            )
                            .with_candidate_key(format!(
                                "{}|{}",
                                prev.candidate_key, rule.candidate_key
                            )),
                        );
                    }

                    if replace {
                        let mut provenance = prev.provenance.clone();
                        merge_provenance(&mut provenance, rule.provenance.clone());
                        bucket.insert(
                            property,
                            PropEntry {
                                decl,
                                order: rule.order.clone(),
                                candidate_key: rule.candidate_key.clone(),
                                provenance,
                            },
                        );
                    } else if same {
                        let mut provenance = prev.provenance.clone();
                        merge_provenance(&mut provenance, rule.provenance.clone());
                        let order = std::cmp::min(prev.order.clone(), rule.order.clone());
                        let candidate_key = if rule.order < prev.order {
                            rule.candidate_key.clone()
                        } else {
                            prev.candidate_key.clone()
                        };
                        bucket.insert(
                            property,
                            PropEntry {
                                decl: prev.decl.clone(),
                                order,
                                candidate_key,
                                provenance,
                            },
                        );
                    }
                }
            }
        }
    }

    // Rebuild one rule per condition bucket.
    let mut rules: Vec<CanonicalRule> = Vec::new();
    for (conditions, props) in buckets {
        if props.is_empty() {
            continue;
        }
        let mut declarations: Vec<Declaration> = Vec::new();
        let mut provenance: Vec<Provenance> = Vec::new();
        let mut order = OrderKey {
            layer: u16::MAX,
            variant_rank: u32::MAX,
            utility_rank: u32::MAX,
            tie: String::new(),
        };
        let mut candidate_key = String::new();

        for (_prop, entry) in props {
            declarations.push(entry.decl);
            merge_provenance(&mut provenance, entry.provenance);
            if entry.order < order || candidate_key.is_empty() {
                if entry.order < order {
                    order = entry.order.clone();
                }
                if candidate_key.is_empty() || entry.candidate_key < candidate_key {
                    candidate_key = entry.candidate_key;
                }
            }
        }

        declarations.sort();
        let mut rule = CanonicalRule {
            candidate_key,
            conditions,
            declarations,
            order,
            provenance,
        };
        rule.order.tie = semantic_tie(&rule);
        rules.push(rule);
    }

    rules.sort_by(|a, b| {
        a.order
            .cmp(&b.order)
            .then_with(|| a.candidate_key.cmp(&b.candidate_key))
            .then_with(|| a.conditions.cmp(&b.conditions))
    });

    let mut theme_keys = referenced_theme_keys;
    theme_keys.sort();
    theme_keys.dedup();

    let content_hash = hash_module(&rules, &theme_keys);

    CanonicalizeOutput {
        module: CanonicalStyleModule {
            rules,
            referenced_theme_keys: theme_keys,
            content_hash,
        },
        diagnostics,
    }
}

fn normalize_rule(mut rule: CanonicalRule) -> CanonicalRule {
    rule.conditions = normalize_conditions(rule.conditions);
    rule.declarations.sort();
    rule.declarations.dedup();
    rule.provenance.sort_by(|a, b| {
        a.source
            .as_str()
            .cmp(b.source.as_str())
            .then_with(|| a.candidate_index.cmp(&b.candidate_index))
    });
    rule.provenance.dedup();
    rule
}

fn normalize_conditions(tree: ConditionTree) -> ConditionTree {
    match tree {
        ConditionTree::Compound(parts) => {
            let mut parts: Vec<_> = parts.into_iter().map(normalize_conditions).collect();
            parts.sort();
            match parts.len() {
                0 => ConditionTree::None,
                1 => parts.remove(0),
                _ => ConditionTree::Compound(parts),
            }
        }
        other => other,
    }
}

fn merge_provenance(dst: &mut Vec<Provenance>, src: Vec<Provenance>) {
    dst.extend(src);
    dst.sort_by(|a, b| {
        a.source
            .as_str()
            .cmp(b.source.as_str())
            .then_with(|| a.candidate_index.cmp(&b.candidate_index))
    });
    dst.dedup();
}

fn semantic_tie(rule: &CanonicalRule) -> String {
    let mut h = StableHasher::new();
    hash_conditions(&mut h, &rule.conditions);
    for d in &rule.declarations {
        hash_declaration(&mut h, d);
    }
    h.finish_hex()
}

fn hash_module(rules: &[CanonicalRule], theme_keys: &[ThemeKey]) -> String {
    let mut h = StableHasher::new();
    h.write_str("tw-canonical-v1");
    h.write_u64(rules.len() as u64);
    for rule in rules {
        hash_rule(&mut h, rule);
    }
    h.write_u64(theme_keys.len() as u64);
    for key in theme_keys {
        h.write_str(&key.joined());
    }
    h.finish_hex()
}

fn hash_rule(h: &mut StableHasher, rule: &CanonicalRule) {
    h.write_str(&rule.candidate_key);
    hash_conditions(h, &rule.conditions);
    h.write_u64(rule.declarations.len() as u64);
    for d in &rule.declarations {
        hash_declaration(h, d);
    }
    hash_order(h, &rule.order);
}

fn hash_order(h: &mut StableHasher, order: &OrderKey) {
    h.write_u16(order.layer);
    h.write_u32(order.variant_rank);
    h.write_u32(order.utility_rank);
    h.write_str(&order.tie);
}

fn hash_conditions(h: &mut StableHasher, tree: &ConditionTree) {
    match tree {
        ConditionTree::None => h.write_u8(0),
        ConditionTree::Selector(s) => {
            h.write_u8(1);
            h.write_str(&s.transform);
        }
        ConditionTree::AtRule(a) => {
            h.write_u8(2);
            h.write_str(&a.name);
            h.write_str(&a.query);
        }
        ConditionTree::Compound(parts) => {
            h.write_u8(3);
            h.write_u64(parts.len() as u64);
            for p in parts {
                hash_conditions(h, p);
            }
        }
    }
}

fn hash_declaration(h: &mut StableHasher, d: &Declaration) {
    h.write_str(&d.property);
    h.write_bool(d.important);
    hash_css_value(h, &d.value);
}

fn hash_css_value(h: &mut StableHasher, v: &tailwind_types::CssValue) {
    use tailwind_types::CssValue::*;
    match v {
        Keyword(s) => {
            h.write_u8(1);
            h.write_str(s);
        }
        Length(l) => {
            h.write_u8(2);
            h.write_str(&l.css);
        }
        Number(s) => {
            h.write_u8(3);
            h.write_str(s);
        }
        Color(c) => {
            h.write_u8(4);
            h.write_str(&c.css);
        }
        Ratio(s) => {
            h.write_u8(5);
            h.write_str(s);
        }
        Shadow(s) => {
            h.write_u8(6);
            h.write_str(s);
        }
        Image(s) => {
            h.write_u8(7);
            h.write_str(s);
        }
        Raw(s) => {
            h.write_u8(8);
            h.write_str(s);
        }
    }
}

fn preview_value(v: &tailwind_types::CssValue) -> String {
    use tailwind_types::CssValue::*;
    match v {
        Keyword(s) | Number(s) | Ratio(s) | Shadow(s) | Image(s) | Raw(s) => s.clone(),
        Length(l) => l.css.clone(),
        Color(c) => c.css.clone(),
    }
}

/// Stable theme snapshot hash (also FNV — not DefaultHasher).
pub fn hash_theme_maps(
    values: &BTreeMap<ThemeKey, tailwind_types::ThemeValue>,
    aliases: &BTreeMap<ThemeKey, ThemeKey>,
    breakpoints: &BTreeMap<String, String>,
) -> String {
    let mut h = StableHasher::new();
    h.write_str("tw-theme-v1");
    for (k, v) in values {
        h.write_str(&k.joined());
        h.write_str(&format!("{v:?}"));
    }
    for (k, v) in aliases {
        h.write_str(&k.joined());
        h.write_str(&v.joined());
    }
    for (k, v) in breakpoints {
        h.write_str(k);
        h.write_str(v);
    }
    h.finish_hex()
}
