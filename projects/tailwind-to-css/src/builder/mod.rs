pub mod parser;
use crate::{TailwindAspect, TailwindBreak, TailwindTableLayout};
pub mod setter;

use crate::{
    BreakPointSystem, CssAttribute, FontSystem, PaletteSystem, PreflightSystem, Result, TailWindZIndex, TailwindInstance,
};
use itertools::Itertools;
use std::{collections::BTreeSet, fmt::Debug};
use tailwind_error::nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::opt, multi::many0, sequence::tuple, IResult,
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
    #[inline]
    #[track_caller]
    pub fn trace(&mut self, style: &str) -> String {
        self.try_trace(style).unwrap()
    }
    pub fn try_trace(&mut self, style: &str) -> Result<String> {
        let parsed = self.parse(style)?;
        let out = parsed.iter().map(|s| s.id()).join(" ");
        // self.buffer.extend(parsed.into_iter());
        for i in parsed.into_iter() {
            self.objects.insert(i);
        }
        return Ok(out);
    }

    pub fn inline(&self, style: &str) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::new();
        let parsed = match self.parse(style) {
            Ok(o) => o,
            Err(_) => return out,
        };
        for item in parsed {
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
