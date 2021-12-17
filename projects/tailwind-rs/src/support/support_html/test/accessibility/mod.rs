use super::*;

#[test]
fn test_accessibility() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("accessibility.html")).unwrap();
    // std::fs::write("src/support/support_html/test/accessibility/accessibility.trace.html", html.as_bytes()).unwrap();
    // std::fs::write("src/support/support_html/test/accessibility/accessibility.trace.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("accessibility.trace.html"));
    assert_eq!(css, include_str!("accessibility.trace.css"));
}
