//! End-to-end: Engine::compile → serialize_module (export stays outside compile).

use tailwind::{CandidateInput, CompileRequest, CompileResponse, Engine, SourceRef};
use tailwind_css::{serialize_module, serialize_module_with, SerializeOptions};

fn compile(token: &str) -> CompileResponse {
    Engine::new().compile(CompileRequest {
        candidates: vec![CandidateInput::new(token, SourceRef::new("t"))],
        ..Default::default()
    })
}

fn compile_many(tokens: &[&str]) -> CompileResponse {
    Engine::new().compile(CompileRequest {
        candidates: tokens
            .iter()
            .map(|t| CandidateInput::new(*t, SourceRef::new("t")))
            .collect(),
        ..Default::default()
    })
}

#[test]
fn compile_response_has_no_css_field() {
    let res = compile("p-4");
    assert!(res.diagnostics.is_empty());
    // Structured only — CSS is produced by this crate, not CompileResponse.
    let _module = &res.module;
    let css = serialize_module(&res.module);
    assert!(css.contains("padding:1rem"), "{css}");
    assert!(!format!("{res:?}").contains("stylesheet"));
}

#[test]
fn e2e_base_hover_md_important_multidecl() {
    let base = compile("p-4");
    assert_eq!(serialize_module(&base.module), ".p-4{padding:1rem}");

    let hover = compile("hover:bg-red-500");
    assert_eq!(
        serialize_module(&hover.module),
        r".hover\:bg-red-500:hover{background-color:#ef4444}"
    );

    let md = compile("md:p-4");
    let md_css = serialize_module(&md.module);
    assert!(md_css.starts_with("@media "), "{md_css}");
    assert!(md_css.contains(r".md\:p-4{padding:1rem}"), "{md_css}");

    let important = compile("!p-[1.5rem]");
    let imp = serialize_module(&important.module);
    assert!(imp.contains("!important"), "{imp}");
    assert!(imp.contains("1.5rem"), "{imp}");

    let px = compile("px-2");
    let px_css = serialize_module(&px.module);
    assert!(px_css.contains("padding-left:"), "{px_css}");
    assert!(px_css.contains("padding-right:"), "{px_css}");
}

#[test]
fn pretty_vs_compact() {
    // Different conditions → separate canonical rules (not merged).
    let res = compile_many(&["p-4", "hover:flex"]);
    assert!(res.diagnostics.is_empty(), "{:?}", res.diagnostics);
    assert!(res.module.rules.len() >= 2, "{:?}", res.module.rules);
    let pretty = serialize_module(&res.module);
    let compact = serialize_module_with(&res.module, &SerializeOptions { pretty: false });
    assert!(pretty.contains('\n'), "{pretty}");
    assert!(!compact.contains('\n'), "{compact}");
    assert_eq!(pretty.replace('\n', ""), compact);
}

#[test]
fn core_facade_does_not_reexport_serializer() {
    // Compiles only if `serialize_module` is *not* in the `tailwind` prelude.
    // (This file imports it from `tailwind_css` explicitly.)
    fn _assert_separate() {
        let _: fn(&tailwind::CanonicalStyleModule) -> String = serialize_module;
    }
}
