use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

use crate::{
    css_attributes, syntax_error, AnchorPoint, CssAttribute, CssBehavior, LengthUnit, Result, TailwindArbitrary,
    TailwindBuilder, TailwindInstance,
};

pub use self::{border_collapse::TailwindBorderCollapse, table_layout::TailwindTableLayout};

mod border_collapse;
mod table_layout;

#[cfg(test)]
mod test;
