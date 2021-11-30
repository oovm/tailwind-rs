use std::collections::HashSet;

use crate::CssProcessor;

pub struct GlobalConfig {
    pub css: CssProcessor,
    pub html: HtmlConfig,
}

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
