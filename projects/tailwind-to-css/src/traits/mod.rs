pub mod attribute;
pub mod instance;

use crate::{Result, TailwindBuilder};
use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
    hash::{Hash, Hasher},
};
use text_utils::indent;

#[allow(unused_variables)]
pub trait TailwindInstance {
    /// used to deduplication and marking
    fn id(&self) -> String;
    /// const ID: &'static str;
    fn selectors(&self, ctx: &TailwindBuilder) -> String {
        format!(".{}", self.id())
    }
    ///
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        BTreeSet::default()
    }
    /// write css to buffers
    fn write_css(&self, f: &mut (dyn Write), ctx: &TailwindBuilder) -> Result<()> {
        writeln!(f, "{} {{", self.selectors(ctx))?;
        for item in self.attributes(ctx) {
            writeln!(f, "{}", indent(item.to_string(), 4))?
        }
        writeln!(f, "}}")?;
        Ok(())
    }
    /// Build css ast
    #[track_caller]
    fn build_css(&self, ctx: &TailwindBuilder) {
        panic!("TODO: build css AST")
    }
}

#[derive(Debug, Clone)]
pub struct CssAttribute {
    key: String,
    value: String,
}
