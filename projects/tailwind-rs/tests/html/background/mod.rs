use super::*;

#[test]
fn test_background_trace() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("background.html"), &mut builder).unwrap();
    std::fs::write("tests/html/background/background.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/background/background.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("background.traced.html"));
    assert_eq!(css, include_str!("background.traced.css"));
}

#[test]
fn test_background_inline() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("background.html"), &mut builder).unwrap();
    std::fs::write("tests/html/background/background.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/background/background.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("background.inline.html"));
    assert_eq!(css, include_str!("background.inline.css"));
}
