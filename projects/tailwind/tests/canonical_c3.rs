use std::panic::{catch_unwind, AssertUnwindSafe};

use tailwind::{
    CandidateInput, CompileRequest, CssValue, DiagnosticCode, Engine, SourceRef,
};

fn req(tokens: &[&str]) -> CompileRequest {
    CompileRequest {
        candidates: tokens
            .iter()
            .enumerate()
            .map(|(i, t)| {
                CandidateInput::new(*t, SourceRef::new(format!("s{i}"))).with_stable_key(*t)
            })
            .collect(),
        ..Default::default()
    }
}

#[test]
fn content_hash_is_stable_across_candidate_reorder_for_same_semantics() {
    // Same utilities under None conditions merge into one canonical rule;
    // input order must not change content_hash.
    let a = Engine::new().compile(req(&["p-4", "block"]));
    let b = Engine::new().compile(req(&["block", "p-4"]));
    assert!(a.diagnostics.is_empty(), "{:?}", a.diagnostics);
    assert!(b.diagnostics.is_empty(), "{:?}", b.diagnostics);
    assert_eq!(a.module.content_hash, b.module.content_hash);
    assert_eq!(a.module.rules.len(), b.module.rules.len());
    assert!(!a.module.content_hash.is_empty());
    // Must look like fixed-width hex (not DefaultHasher randomness).
    assert_eq!(a.module.content_hash.len(), 16);
    assert!(a.module.content_hash.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn duplicate_candidates_dedupe_and_merge_provenance() {
    let res = Engine::new().compile(CompileRequest {
        candidates: vec![
            CandidateInput::new("p-4", SourceRef::new("a")).with_stable_key("k1"),
            CandidateInput::new("p-4", SourceRef::new("b")).with_stable_key("k2"),
        ],
        ..Default::default()
    });
    assert!(res.diagnostics.is_empty(), "{:?}", res.diagnostics);
    assert_eq!(res.module.rules.len(), 1);
    assert_eq!(res.module.rules[0].provenance.len(), 2);
    assert_eq!(
        res.module.rules[0].declarations[0].value,
        CssValue::Length(tailwind::LengthValue {
            css: "1rem".into()
        })
    );
}

#[test]
fn conflicting_padding_emits_canonical_conflict() {
    let res = Engine::new().compile(req(&["p-4", "p-8"]));
    assert!(
        res.diagnostics
            .iter()
            .any(|d| d.code == DiagnosticCode::CanonicalConflict),
        "{:?}",
        res.diagnostics
    );
    // One padding declaration remains (later OrderKey / tie wins).
    let padding = res.module.rules.iter().find_map(|r| {
        r.declarations
            .iter()
            .find(|d| d.property == "padding")
            .cloned()
    });
    assert!(padding.is_some());
}

#[test]
fn variant_rules_stay_separate_from_base() {
    let res = Engine::new().compile(req(&["p-4", "hover:p-4"]));
    assert!(res.diagnostics.is_empty(), "{:?}", res.diagnostics);
    assert_eq!(res.module.rules.len(), 2);
}

#[test]
fn zero_panic_gate_on_garbage_inputs() {
    let engine = Engine::new();
    let long = "a".repeat(10_000);
    let garbage = [
        "",
        "[",
        "((({{{",
        "hover:",
        "::::",
        "p-[",
        "\0p-4",
        "not-hover:md:bg-red-500/9999",
        long.as_str(),
    ];

    for g in garbage {
        let request = CompileRequest {
            candidates: vec![CandidateInput::new(g, SourceRef::new("g"))],
            ..Default::default()
        };
        let result = catch_unwind(AssertUnwindSafe(|| engine.compile(request)));
        assert!(result.is_ok(), "panicked on input {g:?}");
        let response = result.unwrap();
        assert!(
            !response
                .diagnostics
                .iter()
                .any(tailwind::is_internal_panic),
            "internal panic diagnostic for {g:?}: {:?}",
            response.diagnostics
        );
    }
}

#[test]
fn hash_differs_when_semantics_differ() {
    let a = Engine::new().compile(req(&["p-4"]));
    let b = Engine::new().compile(req(&["p-8"]));
    assert_ne!(a.module.content_hash, b.module.content_hash);
}
