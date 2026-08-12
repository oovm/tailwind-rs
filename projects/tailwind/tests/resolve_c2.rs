use tailwind::{
    parse_candidate, CandidateInput, CompileRequest, ConditionTree, CssValue, DiagnosticCode,
    Engine, SourceRef, UtilitySyntax, UtilityValue,
};

fn compile_one(token: &str) -> tailwind::CompileResponse {
    Engine::new().compile(CompileRequest {
        candidates: vec![CandidateInput::new(token, SourceRef::new("t"))],
        ..Default::default()
    })
}

#[test]
fn resolve_spacing_p4() {
    let res = compile_one("p-4");
    assert!(res.diagnostics.is_empty(), "{:?}", res.diagnostics);
    assert_eq!(res.module.rules.len(), 1);
    let rule = &res.module.rules[0];
    assert_eq!(rule.declarations.len(), 1);
    assert_eq!(rule.declarations[0].property, "padding");
    assert_eq!(
        rule.declarations[0].value,
        CssValue::Length(tailwind::LengthValue {
            css: "1rem".into()
        })
    );
    assert!(res
        .module
        .referenced_theme_keys
        .iter()
        .any(|k| k.joined() == "spacing.4"));
}

#[test]
fn resolve_spacing_axes_and_negative_margin() {
    let px = compile_one("px-2");
    assert!(px.diagnostics.is_empty());
    assert_eq!(px.module.rules[0].declarations.len(), 2);

    let neg = compile_one("-m-4");
    assert!(neg.diagnostics.is_empty(), "{:?}", neg.diagnostics);
    assert_eq!(
        neg.module.rules[0].declarations[0].value,
        CssValue::Length(tailwind::LengthValue {
            css: "-1rem".into()
        })
    );

    let bad = compile_one("-p-4");
    assert!(bad
        .diagnostics
        .iter()
        .any(|d| d.code == DiagnosticCode::ResolveNegativeNotAllowed));
}

#[test]
fn resolve_display_hidden_is_none() {
    let res = compile_one("hidden");
    assert!(res.diagnostics.is_empty());
    assert_eq!(
        res.module.rules[0].declarations[0].value,
        CssValue::Keyword("none".into())
    );

    let ib = compile_one("inline-block");
    assert!(ib.diagnostics.is_empty(), "{:?}", ib.diagnostics);
    assert_eq!(
        ib.module.rules[0].declarations[0].value,
        CssValue::Keyword("inline-block".into())
    );
}

#[test]
fn resolve_color_and_variant() {
    let res = compile_one("hover:bg-red-500");
    assert!(res.diagnostics.is_empty(), "{:?}", res.diagnostics);
    let rule = &res.module.rules[0];
    assert_eq!(rule.declarations[0].property, "background-color");
    assert_eq!(
        rule.declarations[0].value,
        CssValue::Color(tailwind::ColorValue {
            css: "#ef4444".into()
        })
    );
    match &rule.conditions {
        ConditionTree::Selector(s) => assert_eq!(s.transform, ":hover"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn resolve_breakpoint_variant() {
    let res = compile_one("md:p-4");
    assert!(res.diagnostics.is_empty(), "{:?}", res.diagnostics);
    match &res.module.rules[0].conditions {
        ConditionTree::AtRule(a) => {
            assert_eq!(a.name, "media");
            assert!(a.query.contains("768px"));
        }
        other => panic!("{other:?}"),
    }
    assert!(res
        .module
        .referenced_theme_keys
        .iter()
        .any(|k| k.joined() == "breakpoints.md"));
}

#[test]
fn resolve_arbitrary_length_and_important() {
    let res = compile_one("!p-[1.5rem]");
    assert!(res.diagnostics.is_empty(), "{:?}", res.diagnostics);
    let d = &res.module.rules[0].declarations[0];
    assert!(d.important);
    assert_eq!(
        d.value,
        CssValue::Length(tailwind::LengthValue {
            css: "1.5rem".into()
        })
    );
}

#[test]
fn resolve_unknown_utility_diagnostic() {
    let res = compile_one("nope-4");
    assert!(res
        .diagnostics
        .iter()
        .any(|d| d.code == DiagnosticCode::ResolveUnknownUtility));
    assert!(res.module.rules.is_empty());
}

#[test]
fn theme_override_changes_spacing() {
    use tailwind::{ThemeEntry, ThemeInput, ThemeKey, ThemeValue};

    let res = Engine::new().compile(CompileRequest {
        candidates: vec![CandidateInput::new("p-4", SourceRef::new("t"))],
        theme: ThemeInput {
            entries: vec![ThemeEntry {
                key: ThemeKey::from_path(["spacing", "4"]),
                value: ThemeValue::Length(tailwind::LengthValue {
                    css: "2rem".into(),
                }),
            }],
        },
        ..Default::default()
    });
    assert!(res.diagnostics.is_empty());
    assert_eq!(
        res.module.rules[0].declarations[0].value,
        CssValue::Length(tailwind::LengthValue {
            css: "2rem".into()
        })
    );
}

#[test]
fn parse_still_works_via_facade() {
    let p4 = parse_candidate("p-4").unwrap();
    match p4.utility {
        UtilitySyntax::Standard {
            value: UtilityValue::Named(v),
            ..
        } => assert_eq!(v.text, "4"),
        other => panic!("{other:?}"),
    }
}
