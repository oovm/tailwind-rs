use crate::CssProcessor;

use super::*;

#[test]
fn go() {
    let mut tw = TailwindBuilder::default();
    let cp = CssProcessor { minify: false, ..Default::default() };
    let input = include_str!("layout.html");
    let new = HtmlConfig::trace_all_class(input, &mut tw).unwrap();
    let out = cp.compile(&tw.try_bundle(input.len()).unwrap()).unwrap();
    assert_eq!(include_str!("layout.trace.css"), out);
    assert_eq!(include_str!("layout.trace.html"), new);
}
