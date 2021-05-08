pub mod parser;
pub mod setter;

use crate::{BreakPointSystem, CssAttribute, FontSystem, PaletteSystem, PreflightSystem, Result, TailwindInstance};
use itertools::Itertools;

use std::{
    collections::{BTreeSet, HashSet},
    fmt::Debug,
};

#[derive(Debug)]
pub struct TailwindBuilder {
    buffer: BTreeSet<Box<dyn TailwindInstance>>,
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
    pub fn trace(&mut self, style: &str) -> String {
        let parsed = self.parse(style);
        let out = parsed.iter().map(|s| s.id()).join(" ");
        // self.buffer.extend(parsed.into_iter());
        for i in parsed.into_iter() {
            self.buffer.insert(i);
        }
        return out;
    }

    pub fn inline(&self, style: &str) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::new();
        for item in self.parse(style) {
            out.extend(item.attributes())
        }
        return out;
    }
    pub fn bundle(&self) -> String {
        self.bundle_buffer(1024 * 10).unwrap_or_default()
    }

    fn bundle_buffer(&self, cap: usize) -> Result<String> {
        let mut out = String::with_capacity(cap);
        self.preflight.write_css(&mut out)?;
        for item in &self.buffer {
            item.write_css(&mut out)?;
        }
        return Ok(out);
    }
}
