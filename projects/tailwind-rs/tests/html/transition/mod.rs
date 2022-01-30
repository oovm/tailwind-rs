use super::*;

#[test]
fn test_transition_trace() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("transition.html"), &mut builder).unwrap();
    std::fs::write("tests/html/transition/transition.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/transition/transition.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("transition.traced.html"));
    assert_eq!(css, include_str!("transition.traced.css"));
}

#[test]
fn test_transition_inline() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("transition.html"), &mut builder).unwrap();
    std::fs::write("tests/html/transition/transition.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/transition/transition.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("transition.inline.html"));
    assert_eq!(css, include_str!("transition.inline.css"));
}
