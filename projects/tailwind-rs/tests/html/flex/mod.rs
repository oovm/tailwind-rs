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

#[test]
fn test_flex_scoped() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_scoped(include_str!("flex.html")).unwrap();
    std::fs::write("tests/html/flex/flex.scoped.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/flex/flex.scoped.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("flex.scoped.html"));
    assert_eq!(css, include_str!("flex.scoped.css"));
}

#[test]
fn test_flex_keyed() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_keyed(include_str!("flex.html")).unwrap();
    std::fs::write("tests/html/flex/flex.keyed.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/flex/flex.keyed.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("flex.keyed.html"));
    assert_eq!(css, include_str!("flex.keyed.css"));
}

#[test]
fn test_flex_value() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_value(include_str!("flex.html")).unwrap();
    std::fs::write("tests/html/flex/flex.value.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/flex/flex.value.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("flex.value.html"));
    assert_eq!(css, include_str!("flex.value.css"));
}
