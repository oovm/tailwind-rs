use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use text_utils::Color;

pub struct ColorMapSystem {
    inner: HashMap<String, ColorMap>,
}

impl Default for ColorMapSystem {
    fn default() -> Self {
        Self {
            inner: Default::default()
        }
    }
}

impl ColorMapSystem {
    pub fn builtin() -> Self {
        let mut new = Self::default();
        new.register("sm".to_string(), 640);
        new.register("md".to_string(), 768);
        new.register("lg".to_string(), 1024);
        new.register("xl".to_string(), 1280);
        new.register("2xl".to_string(), 1536);
        return new
    }

    #[inline]
    pub fn register(&mut self, name: String, width: usize) -> Option<ColorMap> {
        self.inner.insert(name, ColorMap {
            width
        })
    }
}

/// sm	640px	@media (min-width: 640px) { ... }
pub struct ColorMap {
    /// min-width
    /// unit: px
    inner: BTreeMap<usize, Color>,
}

impl Display for ColorMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}