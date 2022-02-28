use super::*;
use crate::CLIConfig;

impl CLIConfig {
    pub fn compile_css(&self, css: &str) -> Result<String> {
        let parser: ParserOptions = ParserOptions {
            //
            filename: "".to_string(),
            nesting: true,
            custom_media: true,
            css_modules: None,
            source_index: 0,
            error_recovery: false,
            warnings: None,
        };
        let mut stylesheet = StyleSheet::parse(css, parser)?;
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
