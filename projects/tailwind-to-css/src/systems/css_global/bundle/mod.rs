use super::*;
mod traits;

#[derive(Debug, Clone)]
pub struct CssBundle {
    inline: Option<InlineMode>,
    items: BTreeSet<CssInstance>,
}

impl CssBundle {
    pub fn insert(&mut self, item: CssInstance) {
        self.items.insert(item);
    }
    pub fn clear(&mut self) {
        self.items.clear()
    }
    pub fn write_css() {}
    pub fn set_inline(&mut self, inline: Option<InlineMode>) {
        self.inline = inline
    }
}
