use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    sync::Arc,
};

use crate::{CssAttributes, Result, TailwindArbitrary, TailwindBuilder, UnimplementedReporter};
pub mod instance;

///
pub trait TailwindProcessor {
    ///
    fn post_processor(&self) -> &[Arc<dyn TailwindProcessor>] {
        &[]
    }
    fn on_catch(&self) -> &'static [&'static str];
    fn on_final(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        UnimplementedReporter {}.on_report(pattern, arbitrary)
    }
    fn on_progress(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        for progress in self.post_processor() {
            let prefix = progress.on_catch();
            if pattern.starts_with(prefix) {
                let rest = &pattern[prefix.len()..pattern.len()];
                return progress.on_progress(rest, arbitrary);
            }
            else {
                continue;
            }
        }
        self.on_final(pattern, arbitrary)
    }
}

impl Debug for dyn TailwindProcessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.on_catch().join("-"))
    }
}

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
    #[allow(unused_variables)]
    fn selectors(&self, ctx: &TailwindBuilder) -> String {
        format!(".{}", self.id())
    }
    /// Attributes in css
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes;
    /// Additional css in bundle
    #[allow(unused_variables)]
    fn additional(&self, ctx: &TailwindBuilder) -> String {
        String::new()
    }
}
