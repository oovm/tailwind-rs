use super::*;
use crate::traits::TailwindProcessor;
use std::sync::Arc;

pub use self::{
    justify_content::TailwindJustifyContent, justify_item::TailwindJustifyItems, justify_self::TailwindJustifySelf,
};

mod justify_content;
mod justify_item;
mod justify_self;

pub struct JustifyAdaptor {
    pub post: Vec<Arc<dyn TailwindProcessor>>,
}

pub struct UnimplementedReport {}

impl TailwindProcessor for UnimplementedReport {
    fn is_registered_word(&self, _: &str) -> bool {
        true
    }

    fn on_process(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        todo!()
    }
}

pub(crate) fn justify_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match str {
        // https://tailwindcss.com/docs/justify-items
        ["items", rest @ ..] => TailwindJustifyItems::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/justify-self
        ["self", rest @ ..] => TailwindJustifySelf::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/justify-content
        _ => TailwindJustifyContent::parse(str, arbitrary)?.boxed(),
    };
    Ok(out)
}

impl TailwindProcessor for JustifyAdaptor {
    fn is_registered_word(&self, word: &str) -> bool {
        ["justify"].contains(word)
    }

    fn on_process(&self, pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/justify-items
            ["items", rest @ ..] => TailwindJustifyItems::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/justify-self
            ["self", rest @ ..] => TailwindJustifySelf::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/justify-content
            _ => TailwindJustifyContent::parse(str, arbitrary)?.boxed(),
        };
        Ok(out)
    }
}

