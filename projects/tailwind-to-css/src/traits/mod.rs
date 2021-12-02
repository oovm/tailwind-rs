use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
    hash::{Hash, Hasher},
};

use crate::{Result, TailwindBuilder};

pub mod attribute;
pub mod instance;

#[allow(unused_variables)]
pub trait TailwindInstance: Display {
    /// used to deduplication and marking
    #[inline]
    fn id(&self) -> String {
        self.to_string()
    }
    /// used to deduplication and marking
    fn inlineable(&self) -> bool {
        true
    }
    fn boxed(self) -> Box<dyn TailwindInstance>
    where
        Self: Sized,
        Self: 'static,
    {
        Box::new(self)
    }
    /// const ID: &'static str;
    fn selectors(&self, ctx: &TailwindBuilder) -> String {
        format!(".{}", self.id())
    }
    ///
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        Default::default()
    }
    /// write css to buffers
    fn write_css(&self, f: &mut (dyn Write), ctx: &TailwindBuilder) -> Result<()> {
        for c in self.selectors(ctx).chars() {
            match c {
                ' ' => write!(f, "_"),
                r @ ('.' | '-' | '_') => write!(f, "{}", r),
                a if a.is_alphanumeric() => write!(f, "{}", a),
                _ => write!(f, "\\{}", c),
            }?
        }
        f.write_char('{')?;
        for item in self.attributes(ctx) {
            write!(f, "{}", item)?
        }
        f.write_char('}')?;
        Ok(())
    }
    /// Build css ast
    #[track_caller]
    fn build_css(&self, ctx: &TailwindBuilder) {
        panic!("TODO: build css AST")
    }
}

///
#[derive(Clone)]
pub struct CssAttribute {
    key: String,
    value: String,
}
