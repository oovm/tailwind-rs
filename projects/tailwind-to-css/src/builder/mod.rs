pub mod parser;
pub mod setter;

use crate::{BreakPointSystem, CssAttribute, FontSystem, PaletteSystem, PreflightSystem, Result, TailwindInstance};
use itertools::Itertools;

use std::{
    collections::BTreeSet,
    fmt::{Arguments, Debug, Write},
};

#[derive(Debug)]
pub struct TailwindBuilder {
    objects: BTreeSet<Box<dyn TailwindInstance>>,
    preflight: PreflightSystem,
    screens: BreakPointSystem,
    colors: PaletteSystem,
    fonts: FontSystem,
}

impl TailwindBuilder {
    #[inline]
    pub fn clear(&mut self) {
        self.objects.clear()
    }
    #[track_caller]
    pub fn trace(&mut self, style: &str) -> String {
        let parsed = self.parse(style);
        let out = parsed.iter().map(|s| s.id()).join(" ");
        // self.buffer.extend(parsed.into_iter());
        for i in parsed.into_iter() {
            self.objects.insert(i);
        }
        return out;
    }

    pub fn inline(&self, style: &str) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::new();
        for item in self.parse(style) {
            out.extend(item.attributes(&self))
        }
        return out;
    }
    pub fn bundle(&self) -> String {
        self.bundle_objects(1024 * 10).unwrap_or_default()
    }

    fn bundle_objects(&self, cap: usize) -> Result<String> {
        let mut out = String::with_capacity(cap);
        self.preflight.write_css(&mut out, &self)?;
        for item in &self.objects {
            item.write_css(&mut out, &self)?;
        }
        return Ok(out);
    }
}
