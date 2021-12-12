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
        match self.mode {
            InlineMode::Standard => self.items.iter().map(|css| css.selector.as_str()).collect::<Vec<_>>().join(" "),
            InlineMode::Inline =>
                self.items.iter().filter(|css| !css.inlinable).map(|css| css.selector.as_str()).collect::<Vec<_>>().join(" "),
            InlineMode::Scoped => {
                todo!()
            },
            InlineMode::DataKey => {
                todo!()
            },
            InlineMode::DataValue => {
                todo!()
            },
        }
    }
    pub fn get_style(&self) -> String {
        let mut out = String::with_capacity(1024);
        match self.mode {
            InlineMode::Standard => self.items.iter().for_each(|css| css.write_css(&mut out).unwrap_or_default()),
            InlineMode::Inline => {
                todo!()
            },
            InlineMode::Scoped => {
                todo!()
            },
            InlineMode::DataKey => {
                todo!()
            },
            InlineMode::DataValue => {
                todo!()
            },
        }
        out
    }
    pub fn as_traced(&self) {
        for i in &self.items {}
    }
    pub fn as_inlined(&mut self) {}
    pub fn set_inline(&mut self, inline: InlineMode) {
        self.mode = inline
    }
}
