use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
};

use crate::{CssAttributes, TailwindBuilder};

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
    /// New tailwind instance
    fn boxed(self) -> Box<dyn TailwindInstance>
    where
        Self: Sized,
        Self: 'static,
    {
        Box::new(self)
    }
    /// Custom selector name
    fn selectors(&self, ctx: &TailwindBuilder) -> String {
        format!(".{}", self.id())
    }
    /// Attributes in css
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes;
    /// Additional css in bundle
    fn additional(&self, ctx: &TailwindBuilder) -> String {
        String::new()
    }
}
