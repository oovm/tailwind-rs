use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
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
        new.register("sm".to_string(), 640);
        new.register("md".to_string(), 768);
        new.register("lg".to_string(), 1024);
        new.register("xl".to_string(), 1280);
        new.register("2xl".to_string(), 1536);
        return new
    }

    #[inline]
    pub fn register(&mut self, name: String, width: usize) -> Option<Palette> {
        self.inner.insert(name, Palette {
            width
        })
    }
}

/// sm	640px	@media (min-width: 640px) { ... }
pub struct Palette {
    /// min-width
    /// unit: px
    inner: BTreeMap<usize, Color>,
}

/// Builtin colors
/// https://tailwindcss.com/docs/customizing-colors
///
impl Palette {
    pub fn slate() -> Self {
        let mut colors = BTreeMap::default();

    }
}

impl Display for Palette {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}