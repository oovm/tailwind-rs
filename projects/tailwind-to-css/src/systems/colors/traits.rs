use super::*;

impl Default for PaletteSystem {
    fn default() -> Self {
        Self { gradient: false, inner: Default::default() }
    }
}

impl Display for Palette {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
