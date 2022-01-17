use super::*;

mod builtin;

///
#[derive(Clone, Debug, Default)]
pub struct PaletteSystem {
    inner: HashMap<String, Palette>,
}

impl PaletteSystem {
    pub fn try_get_color(&self, name: &str, weight: u32) -> Result<Srgb> {
        match self.inner.get(name) {
            Some(p) => p.get_color(weight),
            None => syntax_error!("no such palette"),
        }
    }

    #[inline]
    pub fn register(&mut self, name: String, colors: Palette) -> Option<Palette> {
        self.inner.insert(name, colors)
    }
}
