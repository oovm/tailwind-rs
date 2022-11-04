mod display;
mod from_str;
mod methods;
#[cfg(test)]
mod tests;
use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign},
};

/// `variant:ast-style(grouped!)!`
#[derive(Clone, Debug, PartialEq)]
pub struct AstGroup {
    /// Is a `!important` group
    pub important: bool,
    ///
    pub head: AstStyle,
    ///
    pub children: Vec<AstGroupItem>,
}

/// One of [`AstGroup`] and [`AstStyle`]
#[derive(Clone, Debug, PartialEq)]
pub enum AstGroupItem {
    /// Is grouped node can be expand
    Grouped(AstGroup),
    /// Is standalone ast node
    Styled(AstStyle),
}

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
    pub arbitrary: Option<String>,
}

/// `-[.+]`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstArbitrary {
    /// The arbitrary value text
    pub arbitrary: String,
}

/// `ast-elements`
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AstElements {
    /// `name-space`
    pub elements: Vec<String>,
}

/// `&`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AstReference {}

/// `!`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AstImportant {}

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
