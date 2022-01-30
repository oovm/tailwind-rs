use super::*;

#[test]
fn test_sizing_trace() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("sizing.html"), &mut builder).unwrap();
    std::fs::write("tests/html/sizing/sizing.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/sizing/sizing.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("sizing.traced.html"));
    assert_eq!(css, include_str!("sizing.traced.css"));
}

#[test]
fn test_sizing_inline() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("sizing.html"), &mut builder).unwrap();
    std::fs::write("tests/html/sizing/sizing.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/sizing/sizing.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("sizing.inline.html"));
    assert_eq!(css, include_str!("sizing.inline.css"));
}
