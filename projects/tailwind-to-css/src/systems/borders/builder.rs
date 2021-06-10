use crate::{systems::borders::TailwindBorderStyle, CssAttribute, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

impl Display for TailwindBorderStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBorderStyle {
    fn selectors(&self, ctx: &TailwindBuilder) -> String {
        todo!()
    }
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}
