use super::*;

#[test]
fn test_flex_trace() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("flex.html")).unwrap();
    std::fs::write("tests/html/flex/flex.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/flex/flex.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("flex.traced.html"));
    assert_eq!(css, include_str!("flex.traced.css"));
}

#[test]
fn test_flex_inline() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_inline(include_str!("flex.html")).unwrap();
    std::fs::write("tests/html/flex/flex.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/flex/flex.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("flex.inline.html"));
    assert_eq!(css, include_str!("flex.inline.css"));
}
