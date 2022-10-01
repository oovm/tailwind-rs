use crate::{traits::TailwindProcessor, Result, TailwindArbitrary, TailwindInstance};
use tailwind_error::{TailwindError, TailwindErrorKind};

pub struct UnimplementedReport {}

impl TailwindProcessor for UnimplementedReport {
    fn get_processor<T, R, P>(&self) -> T
    where
        T: Iterator<Item = R>,
        R: AsRef<P>,
        P: TailwindProcessor,
    {
        unreachable!()
    }
    fn on_catch(&self, word: &[&str]) -> Option<&[&str]> {
        Some(word)
    }
    fn on_final(&self, _: &[&str], _: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        unreachable!()
    }
    fn on_progress(&self, _: &[&str], _: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        Err(TailwindError { kind: Box::new(TailwindErrorKind::Incomplete), file: None, range: None })
    }
}
