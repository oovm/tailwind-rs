use crate::{traits::TailwindProcessor, Result, TailwindArbitrary, TailwindInstance};
use std::sync::Arc;
use tailwind_error::{TailwindError, TailwindErrorKind};

/// UnimplementedReport
#[derive(Debug)]
pub struct UnimplementedReporter {}

impl TailwindProcessor for UnimplementedReporter {
    fn get_processor(&self) -> &[Arc<dyn TailwindProcessor>] {
        &[]
    }
    fn on_catch<'a, 'i>(&'a self, pattern: &'i [&'i str]) -> Option<&'i [&'i str]> {
        Some(pattern)
    }
    fn on_final(&self, _: &[&str], _: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        unreachable!()
    }
    fn on_progress(&self, _: &[&str], _: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        Err(TailwindError { kind: Box::new(TailwindErrorKind::Incomplete), file: None, range: None })
    }
}
