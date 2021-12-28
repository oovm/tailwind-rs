use super::*;

#[test]
fn test_typography_trace() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("typography.html")).unwrap();
    std::fs::write("tests/html/typography/typography.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/typography/typography.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("typography.traced.html"));
    assert_eq!(css, include_str!("typography.traced.css"));
}

#[test]
fn test_typography_inline() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_inline(include_str!("typography.html")).unwrap();
    std::fs::write("tests/html/typography/typography.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/typography/typography.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("typography.inline.html"));
    assert_eq!(css, include_str!("typography.inline.css"));
}

#[test]
fn test_typography_scoped() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_scoped(include_str!("typography.html")).unwrap();
    std::fs::write("tests/html/typography/typography.scoped.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/typography/typography.scoped.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("typography.scoped.html"));
    assert_eq!(css, include_str!("typography.scoped.css"));
}

#[test]
fn test_typography_keyed() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_keyed(include_str!("typography.html")).unwrap();
    std::fs::write("tests/html/typography/typography.keyed.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/typography/typography.keyed.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("typography.keyed.html"));
    assert_eq!(css, include_str!("typography.keyed.css"));
}

#[test]
fn test_typography_value() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_value(include_str!("typography.html")).unwrap();
    std::fs::write("tests/html/typography/typography.value.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/typography/typography.value.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("typography.value.html"));
    assert_eq!(css, include_str!("typography.value.css"));
}
