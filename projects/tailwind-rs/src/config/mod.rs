use std::collections::HashSet;
use tailwind_css::CssInlineMode;

/// The `Tailwind` configuration.
#[derive(Debug, Default)]
pub struct CLIConfig {
    pub dry_run: bool,
    pub minify: bool,
    pub obfuscate: bool,
    pub mode: CssInlineMode,
    pub unused_symbols: HashSet<String>,
    pub html: HtmlConfig,
}

/// The `html` configuration.
#[derive(Clone, Debug)]
pub struct HtmlConfig {
    pub include_attributes: HashSet<String>,
}

impl Default for HtmlConfig {
    fn default() -> Self {
        let mut include_attributes = HashSet::default();
        include_attributes.insert("class".to_string());
        Self { include_attributes }
    }
}
