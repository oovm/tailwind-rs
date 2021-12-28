use super::*;

#[test]
fn test_grouped_trace() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("grouped.html")).unwrap();
    std::fs::write("tests/html/grouped/grouped.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/grouped/grouped.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("grouped.traced.html"));
    assert_eq!(css, include_str!("grouped.traced.css"));
}

#[test]
fn test_grouped_inline() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_inline(include_str!("grouped.html")).unwrap();
    std::fs::write("tests/html/grouped/grouped.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/grouped/grouped.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("grouped.inline.html"));
    assert_eq!(css, include_str!("grouped.inline.css"));
}

#[test]
fn test_grouped_scoped() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_scoped(include_str!("grouped.html")).unwrap();
    std::fs::write("tests/html/grouped/grouped.scoped.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/grouped/grouped.scoped.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("grouped.scoped.html"));
    assert_eq!(css, include_str!("grouped.scoped.css"));
}

#[test]
fn test_grouped_keyed() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_keyed(include_str!("grouped.html")).unwrap();
    std::fs::write("tests/html/grouped/grouped.keyed.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/grouped/grouped.keyed.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("grouped.keyed.html"));
    assert_eq!(css, include_str!("grouped.keyed.css"));
}

#[test]
fn test_grouped_value() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_value(include_str!("grouped.html")).unwrap();
    std::fs::write("tests/html/grouped/grouped.value.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/grouped/grouped.value.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("grouped.value.html"));
    assert_eq!(css, include_str!("grouped.value.css"));
}
