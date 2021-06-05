use crate::{systems::borders::TailwindBorderStyle, CssAttribute, TailwindBuilder, TailwindInstance};
use std::collections::BTreeSet;

impl TailwindInstance for TailwindBorderStyle {
    fn id(&self) -> String {
        todo!()
    }
    fn selectors(&self, ctx: &TailwindBuilder) -> String {
        todo!()
    }
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}
