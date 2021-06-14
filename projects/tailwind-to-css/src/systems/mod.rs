pub mod accessibility;
pub mod background;
pub mod borders;
pub mod breakpoints;
pub mod colors;
pub mod effects;
pub mod filters;
pub mod flexbox;
pub mod fonts;
pub mod layouts;
pub mod preflight;
pub mod sizes;
pub mod spaces;
pub mod tables;
pub mod theme;
pub mod typography;

use crate::{
    css_attributes, parse_integer, syntax_error, traits::CssAttribute, ColorResolver, TailwindBrightness, TailwindBuilder,
    TailwindInstance,
};
use css_style::unit::{percent, px, rem, Length};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::{Debug, Display, Formatter, Write},
    str::FromStr,
};
use tailwind_error::{
    nom::{bytes::complete::tag, IResult},
    Result,
};

/// Tailwind Parsed Result
pub type ParsedItem<'a> = IResult<&'a str, Box<dyn TailwindInstance>>;
/// Tailwind Parsed Result
pub type ParsedList<'a> = IResult<&'a str, HashSet<Box<dyn TailwindInstance>>>;

pub(crate) fn as_list(out: ParsedItem) -> ParsedList {
    match out {
        Ok((rest, o)) => Ok((rest, HashSet::from_iter(vec![o]))),
        Err(e) => Err(e),
    }
}

/// Remove instance from builder
pub(crate) struct SealedRemover(String);

/// Uncategorized tailwind property
#[derive(Debug)]
pub struct TailwindObject {
    pub selector: String,
    pub attributes: BTreeSet<CssAttribute>,
}

impl TailwindObject {
    pub fn new(id: impl Into<String>, css: BTreeSet<CssAttribute>) -> Box<dyn TailwindInstance> {
        Box::new(Self { selector: id.into(), attributes: css })
    }
}

impl Display for TailwindObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.write_css(f, &TailwindBuilder::default()) {
            Ok(_) => Ok(()),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

impl TailwindInstance for TailwindObject {
    fn id(&self) -> String {
        self.selector.to_owned()
    }
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        self.attributes.to_owned()
    }
}

impl TailwindObject {
    pub fn parser<'a>(id: &'static str, css: &'static str) -> impl Fn(&'a str) -> ParsedItem<'a> {
        move |input| match tag(id)(input) {
            Ok((rest, _)) => {
                let lines = css.trim().lines();
                let mut out = BTreeSet::default();
                for i in lines.map(|s| s.trim()) {
                    if let Some((key, value)) = i.split_once(":") {
                        out.insert(CssAttribute::new(key, value));
                    }
                }
                Ok((rest, Self::new(id, out)))
            }
            Err(e) => Err(e),
        }
    }
}

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
