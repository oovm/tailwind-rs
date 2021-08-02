mod display;
mod methods;
mod parse;
#[cfg(test)]
mod tests;
use nom::{
    bytes::complete::tag, character::complete::char, combinator::opt, sequence::tuple, IResult,
};

/// `variant:ast-style(grouped)`
#[derive(Clone, Debug, PartialEq)]
pub struct AstGroup<'a> {
    ///
    pub head: AstStyle<'a>,
    ///
    pub children: Vec<AstGroupItem<'a>>,
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum AstGroupItem<'a> {
    ///
    Grouped(AstGroup<'a>),
    ///
    Styled(AstStyle<'a>),
    // SelfReference(AstReference),
}

/// `not-variant:pseudo::-ast-element-[arbitrary]`
#[derive(Clone, Debug, PartialEq)]
pub struct AstStyle<'a> {
    ///
    pub negative: bool,
    ///
    pub variants: Vec<ASTVariant<'a>>,
    ///
    pub elements: Vec<&'a str>,
    ///
    pub arbitrary: Option<&'a str>,
}

/// `-[.+]`
#[derive(Clone, Debug, PartialEq)]
pub struct AstArbitrary<'a> {
    /// `[.]`
    pub arbitrary: &'a str,
}

/// `ast-elements`
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AstElements<'a> {
    /// `name-space`
    pub elements: Vec<&'a str>,
}

/// `&`
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AstReference {}

/// `(not-)?variant:pseudo::`
#[derive(Clone, Debug, PartialEq)]
pub struct ASTVariant<'a> {
    /// `not-`
    pub not: bool,
    /// `::`
    pub pseudo: bool,
    /// `name-space`
    pub names: Vec<&'a str>,
}
