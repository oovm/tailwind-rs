use std::collections::HashSet;
use tailwind_css::TailwindBuilder;

#[derive(Debug, Default)]
pub struct GlobalConfig {
    pub css: CssProcessor,
    pub html: HtmlConfig,
    pub tailwind: TailwindBuilder,
}

///
#[derive(Clone, Debug, Default)]
pub struct CssProcessor {
    pub minify: bool,
    pub unused_symbols: HashSet<String>,
}

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
