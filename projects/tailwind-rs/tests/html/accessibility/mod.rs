use super::*;

#[test]
fn test_accessibility() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("accessibility.html"), &mut builder).unwrap();
    std::fs::write("tests/html/accessibility/accessibility.trace.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/accessibility/accessibility.trace.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("accessibility.trace.html"));
    assert_eq!(css, include_str!("accessibility.trace.css"));
}

#[test]
fn test_obfuscate() {
    let (mut config, mut builder) = pre_config();
    config.obfuscate = true;
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("accessibility.html"), &mut builder).unwrap();
    std::fs::write("tests/html/accessibility/accessibility.obfuscate.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/accessibility/accessibility.obfuscate.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("accessibility.obfuscate.html"));
    assert_eq!(css, include_str!("accessibility.obfuscate.css"));
}
