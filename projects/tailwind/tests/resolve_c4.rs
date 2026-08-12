//! C4 coverage: one utility per major family via the `tailwind` facade.

use tailwind::{
    CandidateInput, CompileRequest, CssValue, Engine, LengthValue, SourceRef,
};

fn compile_one(token: &str) -> tailwind::CompileResponse {
    Engine::new().compile(CompileRequest {
        candidates: vec![CandidateInput::new(token, SourceRef::new("t"))],
        ..Default::default()
    })
}

fn assert_ok(token: &str) -> tailwind::CompileResponse {
    let res = compile_one(token);
    assert!(
        res.diagnostics.is_empty(),
        "{token}: {:?}",
        res.diagnostics
    );
    assert_eq!(res.module.rules.len(), 1, "{token}");
    res
}

fn first_prop(token: &str) -> String {
    assert_ok(token).module.rules[0].declarations[0]
        .property
        .clone()
}

fn first_css(token: &str) -> CssValue {
    assert_ok(token).module.rules[0].declarations[0]
        .value
        .clone()
}

#[test]
fn family_sizing_w4() {
    assert_eq!(
        first_css("w-4"),
        CssValue::Length(LengthValue {
            css: "1rem".into()
        })
    );
    assert_eq!(first_prop("w-4"), "width");
}

#[test]
fn family_flex_row() {
    assert_eq!(
        first_css("flex-row"),
        CssValue::Keyword("row".into())
    );
    assert_eq!(first_prop("flex-row"), "flex-direction");
}

#[test]
fn family_overflow_hidden() {
    assert_eq!(first_prop("overflow-hidden"), "overflow");
    assert_eq!(
        first_css("overflow-hidden"),
        CssValue::Keyword("hidden".into())
    );
    assert_eq!(first_prop("overflow-x-hidden"), "overflow-x");
    assert_eq!(
        first_css("overflow-x-hidden"),
        CssValue::Keyword("hidden".into())
    );
    assert_eq!(first_prop("overflow-y-scroll"), "overflow-y");
    assert_eq!(
        first_css("overflow-y-scroll"),
        CssValue::Keyword("scroll".into())
    );
}

#[test]
fn family_position_absolute() {
    assert_eq!(first_prop("absolute"), "position");
    assert_eq!(first_css("absolute"), CssValue::Keyword("absolute".into()));
}

#[test]
fn family_z_index() {
    assert_eq!(first_prop("z-10"), "z-index");
}

#[test]
fn family_gap() {
    assert_eq!(first_prop("gap-4"), "gap");
    assert_eq!(
        first_css("gap-4"),
        CssValue::Length(LengthValue {
            css: "1rem".into()
        })
    );
}

#[test]
fn family_rounded() {
    assert_eq!(first_prop("rounded-md"), "border-radius");
}

#[test]
fn family_border() {
    let res = assert_ok("border");
    assert_eq!(res.module.rules[0].declarations[0].property, "border-width");
}

#[test]
fn family_shadow() {
    assert_eq!(first_prop("shadow"), "box-shadow");
}

#[test]
fn family_opacity() {
    assert_eq!(first_prop("opacity-50"), "opacity");
}

#[test]
fn family_truncate() {
    let res = assert_ok("truncate");
    assert!(res.module.rules[0].declarations.len() >= 3);
}

#[test]
fn family_sr_only() {
    let res = assert_ok("sr-only");
    assert!(res.module.rules[0].declarations.len() >= 5);
}

#[test]
fn family_rotate() {
    assert_eq!(first_prop("rotate-45"), "transform");
    assert_eq!(
        first_css("rotate-45"),
        CssValue::Raw("rotate(45deg)".into())
    );
}

#[test]
fn family_duration() {
    assert_eq!(first_prop("duration-150"), "transition-duration");
}

#[test]
fn family_fill() {
    assert_eq!(first_prop("fill-black"), "fill");
    assert_eq!(
        first_css("fill-black"),
        CssValue::Color(tailwind::ColorValue {
            css: "#000000".into()
        })
    );
}

#[test]
fn family_cursor() {
    assert_eq!(first_prop("cursor-pointer"), "cursor");
    assert_eq!(
        first_css("cursor-pointer"),
        CssValue::Keyword("pointer".into())
    );
}

#[test]
fn family_table_auto() {
    assert_eq!(first_prop("table-auto"), "table-layout");
    assert_eq!(first_css("table-auto"), CssValue::Keyword("auto".into()));
}

#[test]
fn family_blur() {
    assert_eq!(first_prop("blur-sm"), "filter");
    assert_eq!(
        first_css("blur-sm"),
        CssValue::Raw("blur(4px)".into())
    );
}

#[test]
fn family_min_w_full() {
    assert_eq!(first_prop("min-w-full"), "min-width");
    assert_eq!(
        first_css("min-w-full"),
        CssValue::Length(tailwind_types::LengthValue {
            css: "100%".into(),
        })
    );
}

#[test]
fn family_text_size_and_color() {
    assert_eq!(first_prop("text-sm"), "font-size");
    assert_eq!(first_prop("text-red-500"), "color");
}

#[test]
fn family_display_still_works() {
    assert_eq!(
        first_css("hidden"),
        CssValue::Keyword("none".into())
    );
    assert_eq!(first_prop("flex"), "display");
}
