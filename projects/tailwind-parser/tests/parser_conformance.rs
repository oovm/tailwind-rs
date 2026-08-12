use tailwind_parser::{parse_candidate, parse_source, Lexer, TokenKind};

#[test]
fn lexer_emits_basic_punct() {
    let mut lx = Lexer::new("hover:p-4!");
    let mut kinds = Vec::new();
    loop {
        let tok = lx.next_token().unwrap();
        if tok.kind == TokenKind::Eof {
            break;
        }
        kinds.push(tok.kind);
    }
    assert!(kinds.contains(&TokenKind::Colon));
    assert!(kinds.contains(&TokenKind::Dash));
    assert!(kinds.contains(&TokenKind::Bang));
}

#[test]
fn type_hint_and_css_var() {
    let c = parse_candidate("text-[length:var(--x)]").unwrap();
    match c.utility {
        tailwind_ast::UtilitySyntax::Standard {
            value: tailwind_ast::UtilityValue::Arbitrary(a),
            ..
        } => {
            assert_eq!(a.type_hint.as_ref().unwrap().text, "length");
            assert_eq!(a.raw, "var(--x)");
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn no_panic_on_garbage() {
    for s in ["", "[", "((( ", "hover:", ":::"] {
        let _ = parse_source(s);
        let _ = parse_candidate(s);
    }
}
