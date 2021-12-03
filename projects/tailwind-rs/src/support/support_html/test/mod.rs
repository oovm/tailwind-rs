use crate::GlobalConfig;

use super::*;

#[test]
fn go() {
    let mut config = GlobalConfig::default();
    config.css.minify = false;
    config.tailwind.preflight.disable = true;
    let (html, css) = config.compile_html_trace(include_str!("layout.html")).unwrap();
    assert_eq!(html, include_str!("layout.trace.html"));
    assert_eq!(css, include_str!("layout.trace.css"))
}

impl GlobalConfig {
    pub fn compile_html_trace(&mut self, input: &str) -> Result<(String, String)> {
        let tw = &mut self.tailwind;
        let html = HtmlConfig::trace_all_class(input, tw)?;
        let bundle = tw.try_bundle(input.len()).unwrap();
        let css = self.css.compile(&bundle)?;
        Ok((html, css))
    }
}
