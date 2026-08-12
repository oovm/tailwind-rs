//! C5: layered cache + theme read-set invalidation + parallel determinism.

use tailwind::{
    CandidateInput, CompileRequest, CssValue, Engine, EngineOptions, LengthValue, SourceRef,
    ThemeEntry, ThemeInput, ThemeKey, ThemeValue,
};

fn req(tokens: &[&str]) -> CompileRequest {
    CompileRequest {
        candidates: tokens
            .iter()
            .map(|t| CandidateInput::new(*t, SourceRef::new("t")))
            .collect(),
        options: EngineOptions {
            collect_stats: true,
        },
        ..Default::default()
    }
}

#[test]
fn parse_and_resolve_cache_hit_on_second_compile() {
    let engine = Engine::new().with_parallel(false);
    let request = req(&["p-4", "flex", "md:p-4"]);

    let first = engine.compile(request.clone());
    assert!(first.diagnostics.is_empty(), "{:?}", first.diagnostics);
    assert_eq!(first.stats.parse_cache_hits, 0);
    assert_eq!(first.stats.resolve_cache_hits, 0);

    let second = engine.compile(request);
    assert!(second.diagnostics.is_empty());
    assert!(second.stats.parse_cache_hits >= 3);
    assert!(second.stats.resolve_cache_hits >= 3);
    assert!(second.stats.module_cache_hits >= 1);
    assert_eq!(first.module.content_hash, second.module.content_hash);
}

#[test]
fn theme_override_invalidates_only_dependent_resolve() {
    let engine = Engine::new().with_parallel(false);

    let base = CompileRequest {
        candidates: vec![
            CandidateInput::new("p-4", SourceRef::new("t")),
            CandidateInput::new("flex", SourceRef::new("t")),
        ],
        options: EngineOptions {
            collect_stats: true,
        },
        ..Default::default()
    };
    let warm = engine.compile(base.clone());
    assert!(warm.diagnostics.is_empty());

    let overridden = CompileRequest {
        theme: ThemeInput {
            entries: vec![ThemeEntry {
                key: ThemeKey::from_path(["spacing", "4"]),
                value: ThemeValue::Length(LengthValue {
                    css: "2rem".into(),
                }),
            }],
        },
        ..base.clone()
    };
    let after = engine.compile(overridden);
    assert!(after.diagnostics.is_empty(), "{:?}", after.diagnostics);
    let pad = after
        .module
        .rules
        .iter()
        .flat_map(|r| r.declarations.iter())
        .find(|d| d.property == "padding")
        .expect("padding declaration");
    assert_eq!(
        pad.value,
        CssValue::Length(LengthValue {
            css: "2rem".into()
        })
    );
    // `flex` does not read spacing.4 — resolve can hit; `p-4` must miss and re-resolve.
    assert!(
        after.stats.resolve_cache_hits >= 1,
        "expected flex resolve hit, stats={:?}",
        after.stats
    );
    assert!(after
        .module
        .rules
        .iter()
        .flat_map(|r| r.declarations.iter())
        .any(|d| d.property == "display"));

    // Unrelated theme key change: both should be able to hit after re-warm.
    let _ = engine.compile(base.clone());
    let color_only = CompileRequest {
        theme: ThemeInput {
            entries: vec![ThemeEntry {
                key: ThemeKey::from_path(["colors", "red", "500"]),
                value: ThemeValue::Color(tailwind::ColorValue {
                    css: "#111111".into(),
                }),
            }],
        },
        ..base
    };
    let color_change = engine.compile(color_only);
    assert!(color_change.diagnostics.is_empty());
    // Neither p-4 nor flex reads colors.red.500 → both resolve hits.
    assert!(
        color_change.stats.resolve_cache_hits >= 2,
        "stats={:?}",
        color_change.stats
    );
    let pad = color_change
        .module
        .rules
        .iter()
        .flat_map(|r| r.declarations.iter())
        .find(|d| d.property == "padding")
        .expect("padding declaration");
    assert_eq!(
        pad.value,
        CssValue::Length(LengthValue {
            css: "1rem".into()
        })
    );
}

#[test]
fn breakpoint_read_set_is_recorded() {
    let res = Engine::new().compile(req(&["md:p-4"]));
    assert!(res.diagnostics.is_empty(), "{:?}", res.diagnostics);
    assert!(res
        .module
        .referenced_theme_keys
        .iter()
        .any(|k| k.joined() == "breakpoints.md"));
    assert!(res
        .module
        .referenced_theme_keys
        .iter()
        .any(|k| k.joined() == "spacing.4"));
}

#[test]
fn parallel_matches_sequential_hash() {
    let tokens = [
        "p-4",
        "m-2",
        "flex",
        "md:p-4",
        "hover:bg-red-500",
        "w-full",
        "gap-2",
        "rounded-md",
        "shadow-sm",
        "opacity-50",
    ];
    let request = req(&tokens);

    let seq = Engine::new()
        .with_parallel(false)
        .compile(request.clone());
    let par = Engine::new().with_parallel(true).compile(request);

    assert!(seq.diagnostics.is_empty(), "{:?}", seq.diagnostics);
    assert!(par.diagnostics.is_empty(), "{:?}", par.diagnostics);
    assert_eq!(seq.module.content_hash, par.module.content_hash);
    assert_eq!(seq.module.rules.len(), par.module.rules.len());
    assert_eq!(seq.module.rules, par.module.rules);
}
