use super::*;
mod traits;

#[derive(Debug, Clone, Default)]
pub struct CssBundle {
    mode: InlineMode,
    items: BTreeSet<CssInstance>,
}

impl CssBundle {
    pub fn insert(&mut self, item: CssInstance) {
        self.items.insert(item);
    }
    pub fn clear(&mut self) {
        self.items.clear()
    }
    pub fn get_class(&self) -> String {
        let mut out = String::with_capacity(1024);
        match self.mode {
            InlineMode::Standard => {},
            InlineMode::Inline => {},
            InlineMode::Scoped => {},
            InlineMode::DataKey => {},
            InlineMode::DataValue => {},
        }
        out
    }
    pub fn get_css(&self) -> String {
        let mut out = String::with_capacity(1024);
        match self.mode {
            InlineMode::Standard => {},
            InlineMode::Inline => {},
            InlineMode::Scoped => {},
            InlineMode::DataKey => {},
            InlineMode::DataValue => {},
        }
        out
    }
    pub fn set_inline(&mut self, inline: InlineMode) {
        self.mode = inline
    }
}
