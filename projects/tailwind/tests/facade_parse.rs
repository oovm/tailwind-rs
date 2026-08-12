use tailwind::{
    parse_candidate, parse_source, CandidateInput, CompileRequest, DiagnosticCode, Engine,
    SourceRef, UtilitySyntax, UtilityValue, VariantBody,
};

#[test]
fn facade_exports_engine_and_parse() {
    let request = CompileRequest {
        candidates: vec![
            CandidateInput::new("p-4", SourceRef::new("host:1")),
            CandidateInput::new("hover:bg-red-500", SourceRef::new("host:2")),
        ],
        ..Default::default()
    };
    let response = Engine::new().compile(request);
    assert_eq!(response.stats.candidate_count, 2);
    assert_eq!(response.module.rules.len(), 2);
    assert!(response.diagnostics.is_empty(), "{:?}", response.diagnostics);
}

#[test]
fn parse_spacing_display_color_shapes() {
    let p4 = parse_candidate("p-4").unwrap();
    match p4.utility {
        UtilitySyntax::Standard { name, value, .. } => {
            assert_eq!(name.text, "p");
            match value {
                UtilityValue::Named(v) => assert_eq!(v.text, "4"),
                other => panic!("{other:?}"),
            }
        }
        other => panic!("{other:?}"),
    }

    let block = parse_candidate("block").unwrap();
    match block.utility {
        UtilitySyntax::Standard {
            value: UtilityValue::Bare,
            name,
            ..
        } => assert_eq!(name.text, "block"),
        other => panic!("{other:?}"),
    }

    let bg = parse_candidate("bg-red-500").unwrap();
    match bg.utility {
        UtilitySyntax::Standard { name, value, .. } => {
            assert_eq!(name.text, "bg");
            match value {
                UtilityValue::Named(v) => assert_eq!(v.text, "red-500"),
                other => panic!("{other:?}"),
            }
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn parse_variants_without_pseudo_semantics() {
    let c = parse_candidate("hover:md:block").unwrap();
    assert_eq!(c.variants.len(), 2);
    match &c.variants[0].body {
        VariantBody::Named { names, double_colon } => {
            assert_eq!(names.len(), 1);
            assert_eq!(names[0].text, "hover");
            assert!(!double_colon);
        }
        other => panic!("{other:?}"),
    }
    // Parser must not decide that hover is a pseudo-class.
}

#[test]
fn parse_arbitrary_nested_and_escape() {
    let c = parse_candidate(r#"p-[var(--x,calc(1px+2px))]"#).unwrap();
    match c.utility {
        UtilitySyntax::Standard {
            value: UtilityValue::Arbitrary(a),
            ..
        } => {
            assert!(a.raw.contains("calc(1px+2px)"));
            assert!(a.type_hint.is_none());
        }
        other => panic!("{other:?}"),
    }

    let quoted = parse_candidate(r#"content-['\]']"#).unwrap();
    match quoted.utility {
        UtilitySyntax::Standard {
            value: UtilityValue::Arbitrary(a),
            ..
        } => assert!(a.raw.contains('\\') || a.raw.contains(']')),
        other => panic!("{other:?}"),
    }
}

#[test]
fn parse_group_desugars() {
    let out = parse_source("hover:(bg-red-500 text-white)").unwrap();
    assert_eq!(out.candidates.len(), 2);
    assert_eq!(out.candidates[0].variants.len(), 1);
    assert_eq!(out.candidates[1].variants.len(), 1);
}

#[test]
fn parse_residual_and_unbalanced_are_diagnostics() {
    let err = parse_candidate("p-[unterminated").unwrap_err();
    let d = err.into_diagnostic("p-[unterminated");
    assert_eq!(d.code, DiagnosticCode::ParseUnbalanced);

    let err = parse_source("").unwrap_err();
    assert_eq!(
        err.into_diagnostic("").code,
        DiagnosticCode::ParseEmpty
    );
}

#[test]
fn negative_important_modifier_fraction() {
    let n = parse_candidate("-m-4").unwrap();
    assert!(n.negative);

    let imp = parse_candidate("!p-4").unwrap();
    assert!(imp.important);

    let frac = parse_candidate("w-1/2").unwrap();
    match frac.utility {
        UtilitySyntax::Standard {
            value: UtilityValue::Named(v),
            ..
        } => assert_eq!(v.text, "1/2"),
        other => panic!("{other:?}"),
    }

    let mod_ = parse_candidate("bg-red-500/50").unwrap();
    match mod_.utility {
        UtilitySyntax::Standard {
            modifier: Some(m), ..
        } => match m.value {
            tailwind::ModifierValue::Named(id) => assert_eq!(id.text, "50"),
            other => panic!("{other:?}"),
        },
        other => panic!("{other:?}"),
    }
}
