use crate::{css_attributes, CssAttributes, Result, StandardValue, TailwindArbitrary, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

pub use self::{border_collapse::TailwindBorderCollapse, table_layout::TailwindTableLayout};

mod border_collapse;
mod table_layout;
