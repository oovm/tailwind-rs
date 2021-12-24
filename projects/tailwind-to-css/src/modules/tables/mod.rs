use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

use crate::{css_attributes, CssAttributes, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance};

pub use self::{border_collapse::TailwindBorderCollapse, table_layout::TailwindTableLayout};

mod border_collapse;
mod table_layout;
