use super::*;

#[test]
fn test_arbitrary_trace() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("arbitrary.html"), &mut builder).unwrap();
    std::fs::write("tests/html/arbitrary/arbitrary.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/arbitrary/arbitrary.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("arbitrary.traced.html"));
    assert_eq!(css, include_str!("arbitrary.traced.css"));
}

#[test]
fn test_arbitrary_inline() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("arbitrary.html"), &mut builder).unwrap();
    std::fs::write("tests/html/arbitrary/arbitrary.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/arbitrary/arbitrary.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("arbitrary.inline.html"));
    assert_eq!(css, include_str!("arbitrary.inline.css"));
}

#[test]
fn test_arbitrary_scoped() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Scoped;
    let (html, css) = config.compile_html(include_str!("arbitrary.html"), &mut builder).unwrap();
    std::fs::write("tests/html/arbitrary/arbitrary.scoped.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/arbitrary/arbitrary.scoped.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("arbitrary.scoped.html"));
    assert_eq!(css, include_str!("arbitrary.scoped.css"));
}

#[test]
fn test_arbitrary_keyed() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::DataKey;
    let (html, css) = config.compile_html(include_str!("arbitrary.html"), &mut builder).unwrap();
    std::fs::write("tests/html/arbitrary/arbitrary.keyed.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/arbitrary/arbitrary.keyed.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("arbitrary.keyed.html"));
    assert_eq!(css, include_str!("arbitrary.keyed.css"));
}

#[test]
fn test_arbitrary_value() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::DataValue;
    let (html, css) = config.compile_html(include_str!("arbitrary.html"), &mut builder).unwrap();
    std::fs::write("tests/html/arbitrary/arbitrary.value.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/arbitrary/arbitrary.value.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("arbitrary.value.html"));
    assert_eq!(css, include_str!("arbitrary.value.css"));
}
