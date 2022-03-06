use lightningcss::stylesheet::ParserFlags;
use lightningcss::targets::Targets;
use super::*;
use crate::CLIConfig;

impl CLIConfig {
    pub fn compile_css(&self, css: &str) -> Result<String> {
        let parser: ParserOptions = ParserOptions {
            //
            filename: "".to_string(),
            css_modules: None,
            source_index: 0,
            error_recovery: false,
            warnings: None,
            flags: ParserFlags::default(),
        };
        let mut stylesheet = StyleSheet::parse(css, parser)?;
        let minify = MinifyOptions { targets: Targets::default(), unused_symbols: self.unused_symbols.to_owned() };
        stylesheet.minify(minify)?;
        let printer = PrinterOptions {
            //
            minify: self.minify,
            source_map: None,
            project_root: None,
            targets: Targets::default(),
            analyze_dependencies: None,
            pseudo_classes: None,
        };
        let css = stylesheet.to_css(printer)?;
        Ok(css.code)
    }
}
