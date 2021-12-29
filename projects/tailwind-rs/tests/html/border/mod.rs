use super::*;

#[test]
fn test_border_trace() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("border.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/border/border.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/border/border.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("border.traced.html"));
    assert_eq!(css, include_str!("border.traced.css"));
}

#[test]
fn test_border_inline() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("border.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/border/border.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/border/border.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("border.inline.html"));
    assert_eq!(css, include_str!("border.inline.css"));
}
