use super::*;

#[test]
fn test_transition_trace() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("transition.html")).unwrap();
    std::fs::write("tests/html/transition/transition.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/transition/transition.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("transition.traced.html"));
    assert_eq!(css, include_str!("transition.traced.css"));
}

#[test]
fn test_transition_inline() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_inline(include_str!("transition.html")).unwrap();
    std::fs::write("tests/html/transition/transition.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/transition/transition.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("transition.inline.html"));
    assert_eq!(css, include_str!("transition.inline.css"));
}
