use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Mutex, MutexGuard};

use rayon::prelude::*;
use tailwind_parser::parse_source;
use tailwind_resolve::{
    canonicalize, default_rule_registry, default_theme, default_variant_registry, resolve_candidate,
    ThemeSnapshot,
};
use tailwind_types::{
    CanonicalStyleModule, CompileRequest, CompileResponse, Diagnostic, DiagnosticCode, EngineStats,
    Provenance, Severity, ThemeKey,
};

use crate::cache::{
    combined_registry_version, hash_resolved_rules, hash_syntax, LayeredCache,
};

/// Neutral TW engine entry point.
///
/// Pipeline: Parse → Resolve → Canonicalize (stable hash / dedupe / conflict).
/// C5: layered caches + parallel per-candidate resolve; merge stays deterministic.
/// `compile` is wrapped in a zero-panic gate: user input must never abort the process.
/// Poisoned cache locks are recovered by resetting the cache (engine stays usable).
#[derive(Debug)]
pub struct Engine {
    rules: tailwind_resolve::RuleRegistry,
    variants: tailwind_resolve::VariantRegistry,
    registry_version: u64,
    cache: Mutex<LayeredCache>,
    parallel: bool,
}

impl Clone for Engine {
    fn clone(&self) -> Self {
        Self {
            rules: self.rules.clone(),
            variants: self.variants.clone(),
            registry_version: self.registry_version,
            cache: Mutex::new(LayeredCache::default()),
            parallel: self.parallel,
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        let rules = default_rule_registry();
        let variants = default_variant_registry();
        let registry_version =
            combined_registry_version(rules.content_version(), variants.content_version());
        Self {
            rules,
            variants,
            registry_version,
            cache: Mutex::new(LayeredCache::default()),
            parallel: true,
        }
    }

    /// Enable/disable parallel candidate resolve (default: on).
    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    /// Drop all layered caches (parse / resolve / module).
    pub fn clear_cache(&self) {
        self.with_cache(|cache| cache.clear());
    }

    /// Conformance-only: poison the cache mutex then recover path must still work.
    /// Not part of the stable host API.
    #[doc(hidden)]
    pub fn __poison_cache_for_conformance(&self) {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let guard = self.cache.lock().expect("cache lock");
            let _guard = guard;
            panic!("intentional cache poison for conformance");
        }));
    }

    /// Lock cache; on poison, reset to empty cache and continue.
    fn with_cache<R>(&self, f: impl FnOnce(&mut LayeredCache) -> R) -> R {
        match self.cache.lock() {
            Ok(mut guard) => f(&mut guard),
            Err(poisoned) => {
                let mut guard: MutexGuard<'_, LayeredCache> = poisoned.into_inner();
                *guard = LayeredCache::default();
                f(&mut guard)
            }
        }
    }

    pub fn compile(&self, request: CompileRequest) -> CompileResponse {
        let candidate_count = request.candidates.len() as u32;
        let contribution_ids: Vec<String> = request
            .contributions
            .items
            .iter()
            .map(|c| c.id.clone())
            .collect();
        match catch_unwind(AssertUnwindSafe(|| self.compile_inner(request))) {
            Ok(response) => response,
            Err(_) => {
                // Recover so the next compile on this Engine can succeed.
                self.clear_cache();
                let mut message = String::from(
                    "internal panic caught by zero-panic gate; please report this input as a bug",
                );
                if candidate_count > 0 {
                    message.push_str(&format!(" ({candidate_count} candidates in request)"));
                }
                if !contribution_ids.is_empty() {
                    message.push_str(&format!(
                        " (contributions present: {})",
                        contribution_ids.join(",")
                    ));
                }
                CompileResponse {
                    module: CanonicalStyleModule::default(),
                    diagnostics: vec![Diagnostic::error(
                        DiagnosticCode::Other("tailwind::internal_panic".into()),
                        message,
                    )],
                    stats: EngineStats {
                        candidate_count,
                        rule_count: 0,
                        diagnostic_count: 1,
                        ..Default::default()
                    },
                }
            }
        }
    }

    fn compile_inner(&self, request: CompileRequest) -> CompileResponse {
        let candidate_count = request.candidates.len() as u32;
        let collect_stats = request.options.collect_stats;
        let mut diagnostics = Vec::new();

        // Contribution protocol is not executed yet — never silent-ignore.
        if !request.contributions.items.is_empty() {
            for c in &request.contributions.items {
                diagnostics.push(Diagnostic::error(
                    DiagnosticCode::ContributionUnsupported,
                    format!(
                        "contribution `{}` (protocol_version={}) is not executed: matcher/value schema/resolver ID/conflict policy are not implemented yet",
                        c.id, c.protocol_version
                    ),
                ));
            }
        }

        let theme = match default_theme(request.theme) {
            Ok(t) => t,
            Err(mut ds) => {
                diagnostics.append(&mut ds);
                ThemeSnapshot::empty()
            }
        };

        self.with_cache(|cache| cache.prepare_theme(&theme));

        let work: Vec<CandidateWork> = if self.parallel && request.candidates.len() > 1 {
            request
                .candidates
                .par_iter()
                .enumerate()
                .map(|(index, candidate)| self.process_candidate(index, candidate, &theme))
                .collect()
        } else {
            request
                .candidates
                .iter()
                .enumerate()
                .map(|(index, candidate)| self.process_candidate(index, candidate, &theme))
                .collect()
        };

        let mut rules_out = Vec::new();
        let mut theme_keys: BTreeSet<ThemeKey> = BTreeSet::new();

        for item in work {
            match item {
                CandidateWork::Parsed {
                    parts,
                    mut parse_diags,
                } => {
                    diagnostics.append(&mut parse_diags);
                    for part in parts {
                        match part.result {
                            Ok(resolved) => {
                                for k in &resolved.theme_reads {
                                    theme_keys.insert(k.clone());
                                }
                                rules_out.push(resolved.rule);
                            }
                            Err(d) => diagnostics.push(d),
                        }
                    }
                }
                CandidateWork::ParseFailed { diagnostic } => diagnostics.push(diagnostic),
            }
        }

        let theme_keys: Vec<_> = theme_keys.into_iter().collect();
        let resolved_hash = hash_resolved_rules(&rules_out, &theme_keys);

        let (module, mut canon_diags) = self.with_cache(|cache| {
            if let Some((module, diags)) = cache.get_module(&resolved_hash) {
                (module, diags)
            } else {
                let canon = canonicalize(rules_out, theme_keys);
                cache.put_module(
                    resolved_hash,
                    canon.module.clone(),
                    canon.diagnostics.clone(),
                );
                (canon.module, canon.diagnostics)
            }
        });

        diagnostics.append(&mut canon_diags);

        diagnostics.sort_by(|a, b| {
            a.code
                .as_str()
                .cmp(b.code.as_str())
                .then_with(|| a.message.cmp(&b.message))
                .then_with(|| a.candidate_key.cmp(&b.candidate_key))
        });

        let hits = self.with_cache(|cache| cache.take_hits());
        let hits = if collect_stats {
            hits
        } else {
            Default::default()
        };

        let rule_count = module.rules.len() as u32;
        let diagnostic_count = diagnostics.len() as u32;
        CompileResponse {
            module,
            diagnostics,
            stats: EngineStats {
                candidate_count,
                rule_count,
                diagnostic_count,
                parse_cache_hits: hits.parse_hits,
                resolve_cache_hits: hits.resolve_hits,
                module_cache_hits: hits.module_hits,
            },
        }
    }

    fn process_candidate(
        &self,
        index: usize,
        candidate: &tailwind_types::CandidateInput,
        theme: &ThemeSnapshot,
    ) -> CandidateWork {
        let base_key = candidate
            .stable_key
            .clone()
            .unwrap_or_else(|| candidate.token.clone());

        let parsed = {
            let cached = self.with_cache(|cache| cache.get_parse(&candidate.token));
            if let Some(hit) = cached {
                hit
            } else {
                let fresh = parse_source(&candidate.token);
                self.with_cache(|cache| {
                    cache.put_parse(candidate.token.clone(), fresh.clone());
                });
                fresh
            }
        };

        match parsed {
            Ok(out) => {
                let mut parts = Vec::with_capacity(out.candidates.len());
                for (sub, syntax) in out.candidates.into_iter().enumerate() {
                    let candidate_key = if sub == 0 {
                        base_key.clone()
                    } else {
                        format!("{base_key}#{sub}")
                    };
                    let provenance = Provenance {
                        source: candidate.source.clone(),
                        candidate_index: Some(index as u32),
                    };
                    let syntax_hash = hash_syntax(&syntax);
                    let resolved = {
                        let cached = self.with_cache(|cache| {
                            cache.get_resolve(
                                &syntax_hash,
                                self.registry_version,
                                &candidate_key,
                                theme,
                            )
                        });
                        if let Some(hit) = cached {
                            hit
                        } else {
                            let fresh = resolve_candidate(
                                &syntax,
                                &candidate_key,
                                provenance,
                                &self.rules,
                                &self.variants,
                                theme,
                            );
                            self.with_cache(|cache| {
                                cache.put_resolve(
                                    syntax_hash,
                                    self.registry_version,
                                    candidate_key.clone(),
                                    theme,
                                    fresh.clone(),
                                );
                            });
                            fresh
                        }
                    };
                    let result = match resolved {
                        Ok(r) => Ok(r),
                        Err(d) if d.candidate_key.is_none() => {
                            Err(d.with_candidate_key(candidate_key.clone()))
                        }
                        Err(d) => Err(d),
                    };
                    parts.push(ResolvedPart {
                        index,
                        sub,
                        result,
                    });
                }
                parts.sort_by_key(|p| (p.index, p.sub));
                CandidateWork::Parsed {
                    parts,
                    parse_diags: Vec::new(),
                }
            }
            Err(err) => CandidateWork::ParseFailed {
                diagnostic: err.into_diagnostic(&candidate.token).with_candidate_key(
                    candidate
                        .stable_key
                        .clone()
                        .unwrap_or_else(|| format!("candidate:{index}")),
                ),
            },
        }
    }
}

#[derive(Debug)]
enum CandidateWork {
    Parsed {
        parts: Vec<ResolvedPart>,
        parse_diags: Vec<Diagnostic>,
    },
    ParseFailed {
        diagnostic: Diagnostic,
    },
}

#[derive(Debug)]
struct ResolvedPart {
    index: usize,
    sub: usize,
    result: Result<tailwind_resolve::ResolvedRule, Diagnostic>,
}

/// Explicit zero-panic gate used by conformance tests.
pub fn compile_no_panic(engine: &Engine, request: CompileRequest) -> CompileResponse {
    engine.compile(request)
}

/// Helper for tests: severity of internal panic diagnostic.
pub fn is_internal_panic(d: &Diagnostic) -> bool {
    matches!(&d.code, DiagnosticCode::Other(s) if s == "tailwind::internal_panic")
        || d.severity == Severity::Error && d.message.contains("zero-panic gate")
}
