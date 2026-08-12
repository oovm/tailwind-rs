//! Layered compile caches (C5).
//!
//! ```text
//! parse_cache:   token_text
//! resolve_cache: syntax_hash + theme_slice_hash + registry_version
//! module_cache:  resolved_rules_hash + ordering_version
//! ```

use std::collections::HashMap;

use tailwind_ast::CandidateSyntax;
use tailwind_parser::{ParseError, ParseOutput};
use tailwind_resolve::{ResolvedRule, StableHasher, ThemeSnapshot, ORDERING_VERSION};
use tailwind_types::{CanonicalRule, CanonicalStyleModule, Diagnostic, ThemeKey};

#[derive(Clone, Debug, Default)]
pub struct CacheHitStats {
    pub parse_hits: u32,
    pub resolve_hits: u32,
    pub module_hits: u32,
}

#[derive(Clone, Debug)]
struct ResolveEntry {
    theme_reads: Vec<ThemeKey>,
    theme_slice_hash: String,
    result: Result<ResolvedRule, Diagnostic>,
}

#[derive(Clone, Debug)]
struct ModuleEntry {
    module: CanonicalStyleModule,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Default)]
pub struct LayeredCache {
    parse: HashMap<String, Result<ParseOutput, ParseError>>,
    /// Keyed by `(syntax_hash, registry_version, candidate_key)`; validated via theme_slice_hash.
    resolve: HashMap<(String, u64, String), ResolveEntry>,
    module: HashMap<(String, u64), ModuleEntry>,
    last_theme: Option<ThemeSnapshot>,
    hits: CacheHitStats,
}

impl LayeredCache {
    pub fn take_hits(&mut self) -> CacheHitStats {
        std::mem::take(&mut self.hits)
    }

    pub fn prepare_theme(&mut self, theme: &ThemeSnapshot) {
        if let Some(prev) = &self.last_theme {
            if prev.content_hash != theme.content_hash {
                let changed = theme.changed_keys_from(prev);
                self.invalidate_resolve_for_theme_keys(&changed);
                self.module.clear();
            }
        }
        self.last_theme = Some(theme.clone());
    }

    pub fn invalidate_resolve_for_theme_keys(&mut self, changed: &[ThemeKey]) {
        if changed.is_empty() {
            return;
        }
        let changed_set: std::collections::BTreeSet<_> = changed.iter().cloned().collect();
        self.resolve.retain(|_, entry| {
            !entry
                .theme_reads
                .iter()
                .any(|k| changed_set.contains(k))
        });
    }

    pub fn get_parse(&mut self, token: &str) -> Option<Result<ParseOutput, ParseError>> {
        let hit = self.parse.get(token).cloned();
        if hit.is_some() {
            self.hits.parse_hits = self.hits.parse_hits.saturating_add(1);
        }
        hit
    }

    pub fn put_parse(&mut self, token: String, value: Result<ParseOutput, ParseError>) {
        self.parse.insert(token, value);
    }

    pub fn get_resolve(
        &mut self,
        syntax_hash: &str,
        registry_version: u64,
        candidate_key: &str,
        theme: &ThemeSnapshot,
    ) -> Option<Result<ResolvedRule, Diagnostic>> {
        let key = (
            syntax_hash.to_string(),
            registry_version,
            candidate_key.to_string(),
        );
        let entry = self.resolve.get(&key)?;
        let slice = theme.slice_hash(&entry.theme_reads);
        if slice != entry.theme_slice_hash {
            return None;
        }
        self.hits.resolve_hits = self.hits.resolve_hits.saturating_add(1);
        Some(entry.result.clone())
    }

    pub fn put_resolve(
        &mut self,
        syntax_hash: String,
        registry_version: u64,
        candidate_key: String,
        theme: &ThemeSnapshot,
        result: Result<ResolvedRule, Diagnostic>,
    ) {
        let theme_reads = match &result {
            Ok(r) => r.theme_reads.clone(),
            Err(_) => Vec::new(),
        };
        let theme_slice_hash = theme.slice_hash(&theme_reads);
        self.resolve.insert(
            (syntax_hash, registry_version, candidate_key),
            ResolveEntry {
                theme_reads,
                theme_slice_hash,
                result,
            },
        );
    }

    pub fn get_module(
        &mut self,
        resolved_hash: &str,
    ) -> Option<(CanonicalStyleModule, Vec<Diagnostic>)> {
        let key = (resolved_hash.to_string(), ORDERING_VERSION);
        let hit = self.module.get(&key).cloned();
        if hit.is_some() {
            self.hits.module_hits = self.hits.module_hits.saturating_add(1);
        }
        hit.map(|e| (e.module, e.diagnostics))
    }

    pub fn put_module(
        &mut self,
        resolved_hash: String,
        module: CanonicalStyleModule,
        diagnostics: Vec<Diagnostic>,
    ) {
        self.module.insert(
            (resolved_hash, ORDERING_VERSION),
            ModuleEntry {
                module,
                diagnostics,
            },
        );
    }

    pub fn clear(&mut self) {
        self.parse.clear();
        self.resolve.clear();
        self.module.clear();
        self.last_theme = None;
        self.hits = CacheHitStats::default();
    }
}

pub fn hash_syntax(syntax: &CandidateSyntax) -> String {
    // Semantic fields only — drop spans so re-parses of the same token share keys.
    let mut h = StableHasher::new();
    h.write_str("tw-syntax-v1");
    h.write_bool(syntax.important);
    h.write_bool(syntax.negative);
    hash_utility(&mut h, &syntax.utility);
    h.write_u64(syntax.variants.len() as u64);
    for v in &syntax.variants {
        h.write_bool(v.not);
        match &v.body {
            tailwind_ast::VariantBody::Named {
                names,
                double_colon,
            } => {
                h.write_u8(1);
                h.write_bool(*double_colon);
                h.write_u64(names.len() as u64);
                for n in names {
                    h.write_str(&n.text);
                }
            }
            tailwind_ast::VariantBody::Arbitrary(a) => {
                h.write_u8(2);
                if let Some(hint) = &a.type_hint {
                    h.write_u8(1);
                    h.write_str(&hint.text);
                } else {
                    h.write_u8(0);
                }
                h.write_str(&a.raw);
            }
        }
    }
    h.finish_hex()
}

fn hash_utility(h: &mut StableHasher, utility: &tailwind_ast::UtilitySyntax) {
    match utility {
        tailwind_ast::UtilitySyntax::Standard {
            name,
            value,
            modifier,
        } => {
            h.write_u8(1);
            h.write_str(&name.text);
            match value {
                tailwind_ast::UtilityValue::Bare => h.write_u8(0),
                tailwind_ast::UtilityValue::Named(n) => {
                    h.write_u8(1);
                    h.write_str(&n.text);
                }
                tailwind_ast::UtilityValue::Arbitrary(a) => {
                    h.write_u8(2);
                    if let Some(hint) = &a.type_hint {
                        h.write_u8(1);
                        h.write_str(&hint.text);
                    } else {
                        h.write_u8(0);
                    }
                    h.write_str(&a.raw);
                }
            }
            match modifier {
                None => h.write_u8(0),
                Some(m) => {
                    h.write_u8(1);
                    match &m.value {
                        tailwind_ast::ModifierValue::Named(n) => {
                            h.write_u8(1);
                            h.write_str(&n.text);
                        }
                        tailwind_ast::ModifierValue::Arbitrary(a) => {
                            h.write_u8(2);
                            h.write_str(&a.raw);
                        }
                    }
                }
            }
        }
        tailwind_ast::UtilitySyntax::ArbitraryProperty {
            property,
            value,
            modifier,
        } => {
            h.write_u8(2);
            h.write_str(&property.text);
            if let Some(hint) = &value.type_hint {
                h.write_u8(1);
                h.write_str(&hint.text);
            } else {
                h.write_u8(0);
            }
            h.write_str(&value.raw);
            match modifier {
                None => h.write_u8(0),
                Some(m) => {
                    h.write_u8(1);
                    match &m.value {
                        tailwind_ast::ModifierValue::Named(n) => {
                            h.write_u8(1);
                            h.write_str(&n.text);
                        }
                        tailwind_ast::ModifierValue::Arbitrary(a) => {
                            h.write_u8(2);
                            h.write_str(&a.raw);
                        }
                    }
                }
            }
        }
    }
}

pub fn hash_resolved_rules(rules: &[CanonicalRule], theme_keys: &[ThemeKey]) -> String {
    let mut h = StableHasher::new();
    h.write_str("tw-resolved-v1");
    h.write_u64(ORDERING_VERSION);
    h.write_u64(rules.len() as u64);
    for r in rules {
        h.write_str(&r.candidate_key);
        h.write_str(&format!("{:?}", r.conditions));
        h.write_str(&format!("{:?}", r.declarations));
        h.write_str(&format!("{:?}", r.order));
        h.write_str(&format!("{:?}", r.provenance));
    }
    h.write_u64(theme_keys.len() as u64);
    for k in theme_keys {
        h.write_str(&k.joined());
    }
    h.finish_hex()
}

pub fn combined_registry_version(
    rules_version: u64,
    variants_version: u64,
) -> u64 {
    let mut h = StableHasher::new();
    h.write_str("tw-registry-v1");
    h.write_u64(rules_version);
    h.write_u64(variants_version);
    h.finish()
}
