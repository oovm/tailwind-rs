mod parse;
#[cfg(test)]
mod tests;
// mod tests;
use nom::{bytes::complete::tag, character::complete::char, combinator::opt, sequence::tuple, IResult};

pub fn parse_tailwind(input: &str) -> AstNode {
    todo!()
}

///
#[derive(Clone, Debug)]
pub enum AstNode<'a> {
    Root(Vec<AstNode<'a>>),
    Styled(AstStyle<'a>),
    /// `[.]`
    Arbitrary(AstArbitrary<'a>),
    SelfReference(AstReference),
    Grouped {
        variants: Vec<ASTVariant<'a>>,
        children: Vec<AstNode<'a>>,
    },
}

///
#[derive(Clone, Debug)]
pub struct AstStyle<'a> {
    pub negative: bool,
    pub variants: Vec<ASTVariant<'a>>,
    pub elements: Vec<&'a str>,
    pub arbitrary: Option<AstArbitrary<'a>>,
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct AstArbitrary<'a> {
    pub arbitrary: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
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
