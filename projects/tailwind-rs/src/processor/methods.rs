use super::*;
use crate::CLIConfig;

impl CLIConfig {
    pub fn compile_css(&self, css: &str) -> Result<String> {
        const PARSER: ParserOptions = ParserOptions {
            //
            nesting: true,
            custom_media: true,
            css_modules: false,
            source_index: 0,
        };
        let mut stylesheet = StyleSheet::parse(String::new(), css, PARSER)?;
        let minify = MinifyOptions { targets: None, unused_symbols: self.unused_symbols.to_owned() };
        stylesheet.minify(minify)?;
        let printer = PrinterOptions {
            //
            minify: self.minify,
            source_map: None,
            targets: None,
            analyze_dependencies: false,
            pseudo_classes: None,
        };
        let css = stylesheet.to_css(printer)?;
        Ok(css.code)
    }
}
