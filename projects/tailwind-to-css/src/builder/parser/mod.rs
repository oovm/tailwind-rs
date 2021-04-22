use crate::{CssInstance, TailwindBuilder};
use std::collections::HashSet;

impl TailwindBuilder {
    #[track_caller]
    pub(super) fn parse(input: &str) -> HashSet<Box<dyn CssInstance>> {
        todo!()
    }
}
