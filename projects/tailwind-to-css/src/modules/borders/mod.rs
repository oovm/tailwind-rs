use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::{
    css_attributes, syntax_error, CssAttribute, LengthUnit, Result, TailwindArbitrary, TailwindBorderCollapse, TailwindBuilder,
    TailwindColor, TailwindInstance,
};

pub use self::{border::*, divide::*, outline::*, ring::*};

mod border;
mod divide;
mod outline;
mod ring;
#[cfg(test)]
mod test;

#[derive(Copy, Clone, Debug)]
enum BorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    Hidden,
}

impl Display for BorderStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Solid => write!(f, "solid"),
            Self::Dashed => write!(f, "dashed"),
            Self::Dotted => write!(f, "dotted"),
            Self::Double => write!(f, "double"),
            Self::Hidden => write!(f, "hidden"),
            Self::None => write!(f, "none"),
        }
    }
}
