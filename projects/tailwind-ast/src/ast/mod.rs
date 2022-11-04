mod display;
// mod from_str;
mod methods;

use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

/// `variant:ast-style(grouped!)!`
/// `not-variant:pseudo::-ast-element-[arbitrary]`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstStyle {
    /// Is this `!important` style, end with `!`
    pub important: bool,
    /// Variants for the style, end with `:` or `::`
    pub variants: BTreeSet<ASTVariant>,
    /// Elements for the style, connect with `-`
    pub elements: AstElements,
    /// Is this a arbitrary value, paired with `[` and `]`
    pub arbitrary: AstArbitrary,
    /// Is this a group, paired with `(` and `)`
    pub children: Vec<AstStyle>,
}

/// `-[.+]`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstArbitrary {
    /// The arbitrary value text
    pub item: String,
}

///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstElements {
    /// Is this negative style, start with `-`
    pub negative: bool,
    /// `e1-e2-e3`
    pub items: Vec<String>,
}

/// `(not-)?variant:pseudo::`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ASTVariant {
    /// `not-`
    pub not: bool,
    /// `::`
    pub pseudo: bool,
    /// `name-space`
    pub names: Vec<String>,
}

impl PartialOrd for ASTVariant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.names.partial_cmp(&other.names)
    }
}

impl Ord for ASTVariant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.names.cmp(&other.names)
    }
}
