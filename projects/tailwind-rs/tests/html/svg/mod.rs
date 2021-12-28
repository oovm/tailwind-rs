use super::*;

#[test]
fn test_svg_trace() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("svg.html")).unwrap();
    std::fs::write("tests/html/svg/svg.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/svg/svg.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("svg.traced.html"));
    assert_eq!(css, include_str!("svg.traced.css"));
}

#[test]
fn test_svg_inline() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_inline(include_str!("svg.html")).unwrap();
    std::fs::write("tests/html/svg/svg.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/svg/svg.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("svg.inline.html"));
    assert_eq!(css, include_str!("svg.inline.css"));
}
