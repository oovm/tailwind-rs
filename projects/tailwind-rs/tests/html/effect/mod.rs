use super::*;

#[test]
fn test_effect_trace() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_traced(include_str!("effect.html")).unwrap();
    std::fs::write("tests/html/effect/effect.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/effect/effect.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("effect.traced.html"));
    assert_eq!(css, include_str!("effect.traced.css"));
}

#[test]
fn test_effect_inline() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_inline(include_str!("effect.html")).unwrap();
    std::fs::write("tests/html/effect/effect.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/effect/effect.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("effect.inline.html"));
    assert_eq!(css, include_str!("effect.inline.css"));
}
