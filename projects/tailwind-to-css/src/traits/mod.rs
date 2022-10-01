use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    sync::Arc,
};

use crate::{CssAttributes, Result, TailwindArbitrary, TailwindBuilder, UnimplementedReport};
pub mod instance;

pub struct ProcessorFactory {
    processors: BTreeMap<String, Arc<dyn TailwindProcessor>>,
}
///
pub trait TailwindProcessor {
    ///
    fn get_processor(&self) -> &[Arc<dyn TailwindProcessor>] {
        &[]
    }
    fn on_catch(&self, pattern: &[&str]) -> Option<&[&str]>;
    fn on_final(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        UnimplementedReport {}.on_progress(pattern, arbitrary)
    }
    fn on_progress(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        for progress in self.get_processor() {
            match progress.on_catch(pattern) {
                None => continue,
                Some(s) => progress.on_progress(s, arbitrary),
            }
        }
        self.on_final(pattern, arbitrary)
    }
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
