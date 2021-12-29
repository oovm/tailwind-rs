use super::*;

#[test]
fn test_transform_trace() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("transform.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/transform/transform.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/transform/transform.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("transform.traced.html"));
    assert_eq!(css, include_str!("transform.traced.css"));
}

#[test]
fn test_transform_inline() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("transform.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/transform/transform.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/transform/transform.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("transform.inline.html"));
    assert_eq!(css, include_str!("transform.inline.css"));
}
