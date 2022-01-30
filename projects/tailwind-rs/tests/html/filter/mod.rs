use super::*;

#[test]
fn test_filter_trace() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("filter.html"), &mut builder).unwrap();
    std::fs::write("tests/html/filter/filter.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/filter/filter.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("filter.traced.html"));
    assert_eq!(css, include_str!("filter.traced.css"));
}

#[test]
fn test_filter_inline() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("filter.html"), &mut builder).unwrap();
    std::fs::write("tests/html/filter/filter.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/filter/filter.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("filter.inline.html"));
    assert_eq!(css, include_str!("filter.inline.css"));
}
