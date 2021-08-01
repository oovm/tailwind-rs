mod methods;
mod parse;
#[cfg(test)]
mod tests;
// mod tests;
use nom::{
    bytes::complete::tag, character::complete::char, combinator::opt, sequence::tuple, IResult,
};

///
#[derive(Clone, Debug, PartialEq)]
pub struct AstGroup<'a> {
    head: AstStyle<'a>,
    children: Vec<AstGroupItem<'a>>,
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum AstGroupItem<'a> {
    Grouped(AstGroup<'a>),
    Styled(AstStyle<'a>),
    // SelfReference(AstReference),
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct AstStyle<'a> {
    pub negative: bool,
    pub variants: Vec<ASTVariant<'a>>,
    pub elements: Vec<&'a str>,
    pub arbitrary: Option<&'a str>,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct AstArbitrary<'a> {
    pub arbitrary: &'a str,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AstElements<'a> {
    pub elements: Vec<&'a str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstReference {}

#[derive(Clone, Debug, PartialEq)]
pub struct ASTVariant<'a> {
    pub not: bool,
    pub pseudo: bool,
    pub names: Vec<&'a str>,
}
