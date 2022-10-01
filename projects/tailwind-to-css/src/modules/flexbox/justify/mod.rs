pub use self::{justify_content::TailwindJustifyContent, justify_item::TailwindJustifyItems, justify_self::TailwindJustifySelf};
use super::*;
use crate::{traits::TailwindProcessor, KeywordInstance, KeywordMap};
use std::{fmt::Debug, sync::Arc};

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
    fn post_processor(&self) -> &[Arc<dyn TailwindProcessor>] {
        &self.post
    }

    fn on_catch(&self) -> &'static [&'static str] {
        &["justify"]
    }
}
