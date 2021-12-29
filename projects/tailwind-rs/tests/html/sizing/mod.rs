use super::*;

#[test]
fn test_sizing_trace() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("sizing.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/sizing/sizing.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/sizing/sizing.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("sizing.traced.html"));
    assert_eq!(css, include_str!("sizing.traced.css"));
}

#[test]
fn test_sizing_inline() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("sizing.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/sizing/sizing.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/sizing/sizing.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("sizing.inline.html"));
    assert_eq!(css, include_str!("sizing.inline.css"));
}
