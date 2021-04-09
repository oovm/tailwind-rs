mod builtin;

use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use text_utils::Color;

pub struct PaletteSystem {
    inner: HashMap<String, Palette>,
}

impl Default for PaletteSystem {
    fn default() -> Self {
        Self {
            inner: Default::default()
        }
    }
}

impl PaletteSystem {
    pub fn builtin() -> Self {
        let mut new = Self::default();
        new.register("slate".to_string(), Palette::slate());
        return new;
    }

    #[inline]
    pub fn register(&mut self, name: String, colors: Palette) -> Option<Palette> {
        self.inner.insert(name, colors)
    }
}

/// sm	640px	@media (min-width: 640px) { ... }
pub struct Palette {
    /// min-width
    /// unit: px
    inner: BTreeMap<usize, Color>,
}


impl Display for Palette {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}