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

fn build() {
    TailwindJustifyItems { kind: () }
}

impl TailwindProcessor for JustifyAdaptor {
    fn get_processor(&self) -> &[Arc<dyn TailwindProcessor>] {
        &self.post
    }
    fn on_catch(&self, pattern: &[&str]) -> Option<&[&str]> {
        match pattern {
            ["justify", rest @ ..] => Some(rest),
            _ => None,
        }
    }
}
