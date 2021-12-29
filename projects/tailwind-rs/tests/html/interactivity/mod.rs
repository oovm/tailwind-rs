use super::*;

#[test]
fn test_interactivity_trace() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("interactivity.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/interactivity/interactivity.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/interactivity/interactivity.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("interactivity.traced.html"));
    assert_eq!(css, include_str!("interactivity.traced.css"));
}

#[test]
fn test_interactivity_inline() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("interactivity.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/interactivity/interactivity.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/interactivity/interactivity.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("interactivity.inline.html"));
    assert_eq!(css, include_str!("interactivity.inline.css"));
}
