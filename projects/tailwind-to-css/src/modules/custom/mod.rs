use crate::{Result, TailwindArbitrary, TailwindInstance};
use tailwind_error::{TailwindError, TailwindErrorKind};

/// UnimplementedReport
#[derive(Debug)]
pub struct UnimplementedReporter {}

impl UnimplementedReporter {
    pub fn on_report(&self, _: &[&str], _: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        Err(TailwindError { kind: Box::new(TailwindErrorKind::Incomplete), file: None, range: None })
    }
}
