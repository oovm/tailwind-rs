use super::*;

#[test]
fn test_sizing_trace() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("sizing.html")).unwrap();
    std::fs::write("tests/html/sizing/sizing.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/sizing/sizing.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("sizing.traced.html"));
    assert_eq!(css, include_str!("sizing.traced.css"));
}

#[test]
fn test_sizing_inline() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_inline(include_str!("sizing.html")).unwrap();
    std::fs::write("tests/html/sizing/sizing.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/sizing/sizing.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("sizing.inline.html"));
    assert_eq!(css, include_str!("sizing.inline.css"));
}
