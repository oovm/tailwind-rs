use std::collections::HashSet;
use tailwind_css::{CssInlineMode, TailwindBuilder};

/// The `Tailwind` configuration.
#[derive(Debug, Default)]
pub struct TailwindState {
    pub dry_run: bool,
    pub css: CssProcessor,
    pub html: HtmlConfig,
    pub builder: TailwindBuilder,
}

/// The `css` configuration.
#[derive(Clone, Debug, Default)]
pub struct CssProcessor {
    pub minify: bool,
    pub mode: CssInlineMode,
    pub unused_symbols: HashSet<String>,
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

impl TailwindState {
    pub fn clear(&mut self) {
        self.builder.clear();
    }
}
