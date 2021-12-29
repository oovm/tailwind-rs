use super::*;

#[test]
fn test_grouped_trace() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("grouped.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/grouped/grouped.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/grouped/grouped.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("grouped.traced.html"));
    assert_eq!(css, include_str!("grouped.traced.css"));
}

#[test]
fn test_grouped_inline() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("grouped.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/grouped/grouped.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/grouped/grouped.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("grouped.inline.html"));
    assert_eq!(css, include_str!("grouped.inline.css"));
}

#[test]
fn test_grouped_scoped() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::Scoped;
    let (html, css) = config.compile_html(include_str!("grouped.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/grouped/grouped.scoped.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/grouped/grouped.scoped.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("grouped.scoped.html"));
    assert_eq!(css, include_str!("grouped.scoped.css"));
}

#[test]
fn test_grouped_keyed() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::DataKey;
    let (html, css) = config.compile_html(include_str!("grouped.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/grouped/grouped.keyed.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/grouped/grouped.keyed.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("grouped.keyed.html"));
    assert_eq!(css, include_str!("grouped.keyed.css"));
}

#[test]
fn test_grouped_value() {
    let (config, mut builder) = pre_config();
    let mode = CssInlineMode::DataValue;
    let (html, css) = config.compile_html(include_str!("grouped.html"), &mut builder, &mode).unwrap();
    std::fs::write("tests/html/grouped/grouped.value.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/grouped/grouped.value.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("grouped.value.html"));
    assert_eq!(css, include_str!("grouped.value.css"));
}
