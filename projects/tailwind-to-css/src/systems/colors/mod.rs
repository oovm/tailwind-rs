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

/// Builtin colors
/// https://tailwindcss.com/docs/customizing-colors
impl Palette {
    ///
    pub fn slate() -> Self {
        let mut colors = BTreeMap::default();
        colors.insert(50, Color::from_str("#F8FAFC").unwrap());
        colors.insert(100, Color::from_str("#f1f5f9").unwrap());
        colors.insert(200, Color::from_str("#e2e8f0").unwrap());
        colors.insert(300, Color::from_str("#cbd5e1").unwrap());
        colors.insert(400, Color::from_str("#94a3b8").unwrap());
        colors.insert(500, Color::from_str("#64748b").unwrap());
        colors.insert(600, Color::from_str("#475569").unwrap());
        colors.insert(700, Color::from_str("#334155").unwrap());
        colors.insert(800, Color::from_str("#1e293b").unwrap());
        colors.insert(900, Color::from_str("#0f172a").unwrap());
        Self { inner: colors }
    }
}

impl Display for Palette {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}