pub mod accessibility;
pub mod background;
pub mod borders;
pub mod breakpoints;
pub mod colors;
pub mod effects;
pub mod filters;
pub mod flexbox;
pub mod fonts;
pub mod interactivity;
pub mod layouts;
pub mod preflight;
pub mod sizing;
pub mod spacing;
pub mod tables;
pub mod theme;
pub mod transition;
pub mod typography;

use crate::{
    css_attributes, parse_integer, syntax_error, traits::CssAttribute, ColorResolver, TailwindBrightness, TailwindBuilder,
    TailwindInstance,
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::{Debug, Display, Formatter, Write},
    str::FromStr,
};
use tailwind_error::Result;

// /// Uncategorized tailwind property
// #[derive(Debug)]
// pub struct TailwindObject {
//     pub selector: String,
//     pub attributes: BTreeSet<CssAttribute>,
// }
//
// impl TailwindObject {
//     pub fn new(id: impl Into<String>, css: BTreeSet<CssAttribute>) -> Box<dyn TailwindInstance> {
//         Box::new(Self { selector: id.into(), attributes: css })
//     }
// }
//
// impl Display for TailwindObject {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self.write_css(f, &TailwindBuilder::default()) {
//             Ok(_) => Ok(()),
//             Err(_) => Err(std::fmt::Error),
//         }
//     }
// }
//
// impl TailwindInstance for TailwindObject {
//     fn id(&self) -> String {
//         self.selector.to_owned()
//     }
//     fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
//         self.attributes.to_owned()
//     }
// }

#[macro_export]
macro_rules! syntax_error {
    ($msg:literal $(,)?) => {
        Err(tailwind_error::TailwindError::syntax_error($msg.to_string()))
    };
    // ($err:expr $(,)?) => {
    //     Err(TailwindError::from($err))
    // };
    ($fmt:expr, $($arg:tt)*) => {
        Err(tailwind_error::TailwindError::syntax_error(format!($fmt, $($arg)*)))
    };
}
