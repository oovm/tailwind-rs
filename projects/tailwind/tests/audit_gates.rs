//! Audit-gate tests: honest failure modes called out in post-C6 review.

use tailwind::{
    is_internal_panic, CandidateInput, CompileRequest, Contribution, ContributionSet,
    DiagnosticCode, Engine, SourceRef,
};

#[test]
fn contributions_are_not_silently_ignored() {
    let res = Engine::new().compile(CompileRequest {
        candidates: vec![CandidateInput::new("p-4", SourceRef::new("t"))],
        contributions: ContributionSet {
            items: vec![Contribution {
                id: "plugin.demo".into(),
                protocol_version: 1,
            }],
        },
        ..Default::default()
    });
    assert!(
        res.diagnostics
            .iter()
            .any(|d| d.code == DiagnosticCode::ContributionUnsupported),
        "{:?}",
        res.diagnostics
    );
    // Built-in utilities still resolve; contributions just don't apply.
    assert!(!res.module.rules.is_empty());
}

#[test]
fn poisoned_cache_lock_recovers_on_next_compile() {
    let engine = Engine::new().with_parallel(false);
    engine.__poison_cache_for_conformance();

    let ok = engine.compile(CompileRequest {
        candidates: vec![CandidateInput::new("flex", SourceRef::new("t"))],
        ..Default::default()
    });
    assert!(
        ok.diagnostics.iter().all(|d| !is_internal_panic(d)),
        "{:?}",
        ok.diagnostics
    );
    assert!(!ok.module.rules.is_empty());
}
