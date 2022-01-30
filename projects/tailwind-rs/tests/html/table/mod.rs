use super::*;

#[test]
fn test_table_trace() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("table.html"), &mut builder).unwrap();
    // std::fs::write("tests/html/table/table.traced.html", html.as_bytes()).unwrap();
    // std::fs::write("tests/html/table/table.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("table.traced.html"));
    assert_eq!(css, include_str!("table.traced.css"));
}

#[test]
fn test_table_inline() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("table.html"), &mut builder).unwrap();
    // std::fs::write("tests/html/table/table.inline.html", html.as_bytes()).unwrap();
    // std::fs::write("tests/html/table/table.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("table.inline.html"));
    assert_eq!(css, include_str!("table.inline.css"));
}
