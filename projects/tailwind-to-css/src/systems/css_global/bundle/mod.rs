use itertools::Itertools;

use super::*;

mod traits;

/// A collection of css objects
///
/// Separate or merge as needed
#[derive(Debug, Clone, Default)]
pub struct CssBundle {
    inline: bool,
    items: BTreeSet<CssInstance>,
}

impl CssBundle {
    /// insert new css instance to the html tag
    pub fn insert(&mut self, item: CssInstance) -> bool {
        self.items.insert(item)
    }
    /// clear all css instance in this html tag
    pub fn clear(&mut self) {
        self.items.clear()
    }
    /// Try inline styles, keep the class name if that fails.
    ///
    /// # Returns
    ///
    /// - css classes
    ///
    /// ```html
    /// <img class="tw-1 tw-2"/>
    /// ```
    pub fn as_traced(&self) -> String {
        debug_assert!(!self.inline);
        self.items.iter().map(|css| css.selector.as_str()).collect::<Vec<_>>().join(" ")
    }
    /// Try inline styles, keep the class name if that fails.
    ///
    /// # Returns
    ///
    /// - `.0`: css classes rest, maybe empty
    /// - `.1`: css style
    ///
    /// ```html
    /// <img class="not-inlinable" style="k1:v1;k2:v2;"/>
    /// ```
    pub fn as_inlined(&self) -> (String, String) {
        debug_assert!(self.inline);
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
    /// # Returns
    ///
    /// - scoped class name
    ///
    /// ```html
    /// <img class="_b2JmdXNjYXRl"/>
    /// ```
    pub fn as_scope(&self) {
        todo!()
    }
    /// # Returns
    ///
    /// - data name without value
    ///
    /// ```html
    /// <img data-tw-b2JmdXNjYXRl/>
    /// ```
    ///
    /// ```css
    /// [data-tw-b2JmdXNjYXRl] {
    ///
    /// }
    /// ```
    pub fn as_data_key(&self) {
        todo!()
    }
    /// # Returns
    ///
    /// - scoped class name
    ///
    /// ```html
    /// <img data-tw="b2JmdXNjYXRl"/>
    /// ```
    ///
    /// ```css
    /// [data-tw="b2JmdXNjYXRl"] {
    ///
    /// }
    /// ```
    pub fn as_data_value(&self) {
        todo!()
    }
    /// Mark inline mode
    pub fn set_inline(&mut self, inline: bool) {
        self.inline = inline
    }
}
