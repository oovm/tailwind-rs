use super::*;

impl Default for CssProcessor {
    fn default() -> Self {
        Self { minify: true, unused_symbols: Default::default() }
    }
}

impl CssProcessor {
    pub fn compile(&self, css: &str) -> Result<String> {
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

#[test]
fn test() {
    let post_processor = CssProcessor { minify: false, unused_symbols: Default::default() };

    assert_eq!(
        post_processor
            .compile(
                r#"
        span {
            --w: 1px,
            width: 1px,
            width: 1px,
        }
        "#
            )
            .unwrap(),
        "span{--w:1px,width: var(--w),width: var(--w),}"
    )
}
