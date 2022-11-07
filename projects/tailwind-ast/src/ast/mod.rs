use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Display, Formatter, Write},
};

use serde::{Deserialize, Serialize};

mod display;
// mod from_str;
mod arbitrary;
mod elements;
mod methods;

/// `variant:ast-style(grouped!)!`
/// `not-variant:pseudo::-ast-element-[arbitrary]`
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstStyle {
    /// Is this `!important` style, end with `!`
    pub important: bool,
    /// Variants for the style, end with `:` or `::`
    pub variants: BTreeSet<TailwindVariant>,
    /// Elements for the style, connect with `-`
    pub elements: TailwindElements,
    /// Is this a arbitrary value, paired with `[` and `]`
    pub arbitrary: TailwindArbitrary,
    /// Is this a group, paired with `(` and `)`
    pub children: Vec<AstStyle>,
}

///
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TailwindElements {
    /// Is this negative style, start with `-`
    pub negative: bool,
    /// `e1-e2-e3`
    pub items: Vec<String>,
}

/// `(not-)?variant:pseudo::`
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TailwindVariant {
    /// `not-`
    pub not: bool,
    /// `::`
    pub pseudo: bool,
    /// `name-space`
    pub names: Vec<String>,
}

///
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TailwindArbitrary {
    inner: Box<str>,
}
