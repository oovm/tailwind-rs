use super::*;
use crate::traits::TailwindProcessor;
use std::sync::Arc;

pub use self::{justify_content::TailwindJustifyContent, justify_item::TailwindJustifyItems, justify_self::TailwindJustifySelf};

mod justify_content;
mod justify_item;
mod justify_self;

///
#[derive(Debug)]
pub struct JustifyAdaptor {
    /// normally
    /// [`JustifyAdaptor`]
    pub post: Vec<Arc<dyn TailwindProcessor>>,
}

impl TailwindProcessor for JustifyAdaptor {
    fn get_processor(&self) -> &[Arc<dyn TailwindProcessor>] {
        &self.post
    }

    fn on_catch<'a, 'i>(&'a self, pattern: &'i [&'i str]) -> Option<&'i [&'i str]> {
        match pattern {
            ["justify", rest @ ..] => Some(rest),
            _ => None,
        }
    }
}
