use itertools::Itertools;

use super::*;

mod traits;

#[derive(Debug, Clone, Default)]
pub struct CssBundle {
    mode: InlineMode,
    items: BTreeSet<CssInstance>,
}

impl CssBundle {
    pub fn insert(&mut self, item: CssInstance) -> bool {
        self.items.insert(item)
    }
    pub fn clear(&mut self) {
        self.items.clear()
    }
    pub fn as_traced(&self) -> String {
        debug_assert!(matches!(self.mode, InlineMode::None));
        self.items.iter().map(|css| css.selector.as_str()).collect::<Vec<_>>().join(" ")
    }
    pub fn as_inlined(&self) -> (String, String) {
        debug_assert!(!matches!(self.mode, InlineMode::None));
        let mut class = BTreeSet::new();
        let mut attribute = BTreeSet::new();
        for i in self.items.iter() {
            match i.inlinable {
                true => attribute.extend(i.attribute.iter()),
                false => {
                    class.insert(i.selector.to_string());
                },
            }
        }
        let class = class.into_iter().join(" ");
        let attribute = attribute.into_iter().join("");
        (class, attribute)
    }
    pub fn set_inline(&mut self, inline: InlineMode) {
        self.mode = inline
    }
}
