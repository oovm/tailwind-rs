use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    sync::Arc,
};

use crate::{CssAttributes, Result, TailwindArbitrary, TailwindBuilder};
pub mod instance;

///
pub trait TailwindProcessor {
    fn id(&self) -> String;
    fn new() -> Arc<dyn TailwindProcessor>;
    /// Whether to capture the required prefix
    fn is_registered_word(&self, word: &str) -> bool;

    fn on_catch(&self) {}
    ///
    fn on_process(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>>;
}

#[allow(unused_variables)]
pub trait TailwindInstance: Display {
    /// Used to deduplication and marking
    #[inline]
    fn id(&self) -> String {
        self.to_string()
    }
    /// Is the instance inlineable
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
