use super::*;

#[test]
fn test_transform_trace() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("transform.html")).unwrap();
    std::fs::write("tests/html/transform/transform.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/transform/transform.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("transform.traced.html"));
    assert_eq!(css, include_str!("transform.traced.css"));
}

#[test]
fn test_transform_inline() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_inline(include_str!("transform.html")).unwrap();
    std::fs::write("tests/html/transform/transform.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/transform/transform.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("transform.inline.html"));
    assert_eq!(css, include_str!("transform.inline.css"));
}
