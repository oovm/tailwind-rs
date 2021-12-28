use super::*;

#[test]
fn test_table_trace() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("table.html")).unwrap();
    // std::fs::write("tests/html/table/table.traced.html", html.as_bytes()).unwrap();
    // std::fs::write("tests/html/table/table.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("table.traced.html"));
    assert_eq!(css, include_str!("table.traced.css"));
}

#[test]
fn test_table_inline() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_inline(include_str!("table.html")).unwrap();
    // std::fs::write("tests/html/table/table.inline.html", html.as_bytes()).unwrap();
    // std::fs::write("tests/html/table/table.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("table.inline.html"));
    assert_eq!(css, include_str!("table.inline.css"));
}
