use super::*;

impl Default for PaletteSystem {
    fn default() -> Self {
        Self { interpolation: false, inner: Default::default() }
    }
}

impl Display for Palette {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
