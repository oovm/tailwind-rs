use super::*;

#[test]
fn test_svg_trace() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("svg.html"), &mut builder).unwrap();
    std::fs::write("tests/html/svg/svg.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/svg/svg.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("svg.traced.html"));
    assert_eq!(css, include_str!("svg.traced.css"));
}

#[test]
fn test_svg_inline() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("svg.html"), &mut builder).unwrap();
    std::fs::write("tests/html/svg/svg.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/svg/svg.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("svg.inline.html"));
    assert_eq!(css, include_str!("svg.inline.css"));
}
