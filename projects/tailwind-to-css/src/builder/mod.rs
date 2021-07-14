mod parser;
mod setter;

pub use self::parser::*;
use crate::*;
use log::error;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
    str::FromStr,
};
use tailwind_error::nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_till1},
    character::complete::{alphanumeric1, char, digit1, multispace1},
    combinator::{map_res, opt, recognize},
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug)]
pub struct TailwindBuilder {
    pub objects: BTreeSet<Box<dyn TailwindInstance>>,
    pub preflight: PreflightSystem,
    pub screens: BreakPointSystem,
    pub colors: PaletteSystem,
    pub fonts: FontSystem,
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
        let parsed = Self::parse_styles(style)?;
        let out: Vec<String> = parsed.iter().map(|s| s.id()).collect();
        for i in parsed.into_iter() {
            self.objects.insert(i.get_instance()?);
        }
        Ok(out.join(" "))
    }

    pub fn inline(&self, style: &str) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::new();
        let parsed = match Self::parse_styles(style) {
            Ok(o) => o,
            Err(_) => return out,
        };
        for item in parsed {
            out.extend(item.attributes(self))
        }
        out
    }
    pub fn bundle(&self) -> String {
        self.bundle_objects(1024 * 10).unwrap_or_default()
    }

    fn bundle_objects(&self, cap: usize) -> Result<String> {
        let mut out = String::with_capacity(cap);
        self.preflight.write_css(&mut out, self)?;
        for item in &self.objects {
            item.write_css(&mut out, self)?;
        }
        Ok(out)
    }
}
