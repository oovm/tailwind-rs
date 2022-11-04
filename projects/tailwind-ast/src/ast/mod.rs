mod display;
// mod from_str;
mod methods;
use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign},
};

/// `variant:ast-style(grouped!)!`
/// `not-variant:pseudo::-ast-element-[arbitrary]`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstStyle {
    /// Is a `!important` style
    pub important: bool,
    /// Is a negative style
    pub negative: bool,
    ///
    pub variants: Vec<ASTVariant>,
    ///
    pub elements: Vec<String>,
    /// Is a arbitrary value
    pub arbitrary: AstArbitrary,
    ///
    pub children: Vec<AstStyle>,
}

/// `-[.+]`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstArbitrary {
    /// The arbitrary value text
    pub arbitrary: String,
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
