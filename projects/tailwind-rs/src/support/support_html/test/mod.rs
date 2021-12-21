use crate::GlobalConfig;

use super::*;
mod accessibility;
mod flex;
mod layout;

impl GlobalConfig {
    pub fn compile_html_traced(&mut self, input: &str) -> Result<(String, String)> {
        let tw = &mut self.tailwind;
        let html = HtmlConfig::trace_all_class(input, tw)?;
        let bundle = tw.bundle()?;
        let css = self.css.compile(&bundle)?;
        Ok((html, css))
    }
    pub fn compile_html_inline(&mut self, input: &str) -> Result<(String, String)> {
        let tw = &mut self.tailwind;
        let html = HtmlConfig::inline_all_class(input, tw)?;
        let bundle = tw.bundle()?;
        let css = self.css.compile(&bundle)?;
        Ok((html, css))
    }
    pub fn compile_html_scoped(&mut self, input: &str) -> Result<(String, String)> {
        let tw = &mut self.tailwind;
        let html = HtmlConfig::scope_all_class(input, tw)?;
        let bundle = tw.bundle()?;
        let css = self.css.compile(&bundle)?;
        Ok((html, css))
    }
    pub fn compile_html_keyed(&mut self, input: &str) -> Result<(String, String)> {
        let tw = &mut self.tailwind;
        let html = HtmlConfig::keyed_all_class(input, tw)?;
        let bundle = tw.bundle()?;
        let css = self.css.compile(&bundle)?;
        Ok((html, css))
    }
    pub fn compile_html_value(&mut self, input: &str) -> Result<(String, String)> {
        let tw = &mut self.tailwind;
        let html = HtmlConfig::value_all_class(input, tw)?;
        let bundle = tw.bundle()?;
        let css = self.css.compile(&bundle)?;
        Ok((html, css))
    }
}
