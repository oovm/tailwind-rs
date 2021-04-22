use crate::{BreakPointSystem, CssInstance, FontSystem, PaletteSystem, PreflightSystem, Result};
use std::{collections::HashSet, fmt::Debug};

#[derive(Debug)]
pub struct TailwindBuilder {
    buffer: HashSet<Box<dyn CssInstance>>,
    preflight: PreflightSystem,
    screens: BreakPointSystem,
    colors: PaletteSystem,
    fonts: FontSystem,
}

impl Default for TailwindBuilder {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            screens: BreakPointSystem::builtin(),
            colors: PaletteSystem::builtin(),
            fonts: FontSystem::builtin(),
            preflight: PreflightSystem::default(),
        }
    }
}

impl TailwindBuilder {
    #[inline]
    pub fn clear(&mut self) {
        self.buffer.clear()
    }
    #[track_caller]
    pub fn trace(&mut self, style: &str) -> Result<String> {
        todo!()
    }
    pub fn build(&self) -> Result<String> {
        let mut out = String::with_capacity(1024 * 10);
        self.preflight.write_css(&mut out)?;

        for item in &self.buffer {
            item.write_css(&mut out)?;
        }
        return Ok(out);
    }
}
