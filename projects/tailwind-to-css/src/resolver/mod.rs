use crate::{BreakPointSystem, CssDisplay, FontSystem, PaletteSystem, PreflightSystem, Result};
use std::collections::HashSet;

pub struct TailwindBuilder {
    config: TailwindConfig,
    buffer: HashSet<String>,
}

impl Default for TailwindBuilder {
    fn default() -> Self {
        Self { config: Default::default(), buffer: Default::default() }
    }
}

impl TailwindBuilder {
    pub fn build() {}
    pub fn trace(&mut self, style: &str) -> Result<String> {}
    pub fn clear(&mut self) {
        self.buffer.clear()
    }
}

#[derive(Clone, Debug)]
pub struct TailwindConfig {
    screens: BreakPointSystem,
    colors: PaletteSystem,
    fonts: FontSystem,
    preflight: PreflightSystem,
}

impl Default for TailwindConfig {
    fn default() -> Self {
        Self {
            screens: BreakPointSystem::builtin(),
            colors: PaletteSystem::builtin(),
            fonts: FontSystem::builtin(),
            preflight: PreflightSystem::default(),
        }
    }
}
