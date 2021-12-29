use super::*;

#[test]
fn test_spacing_trace() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("spacing.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/spacing/spacing.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/spacing/spacing.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("spacing.traced.html"));
    assert_eq!(css, include_str!("spacing.traced.css"));
}

#[test]
fn test_spacing_inline() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("spacing.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/spacing/spacing.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/spacing/spacing.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("spacing.inline.html"));
    assert_eq!(css, include_str!("spacing.inline.css"));
}
