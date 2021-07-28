mod parse;
#[cfg(test)]
mod tests;
// mod tests;
use crate::*;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};

pub fn parse_tailwind(input: &str) -> AstNode {
    todo!()
}

///
#[derive(Clone, Debug)]
pub enum AstNode<'a> {
    Root(Vec<AstNode<'a>>),
    Styled(AstStyle<'a>),
    /// `[.]`
    Arbitrary(&'a str),
    SelfReference,
    Grouped {
        variants: Vec<ASTVariant<'a>>,
        children: Vec<AstNode<'a>>,
    },
}

#[derive(Clone, Debug)]
pub struct AstStyle<'a> {
    pub negative: bool,
    pub variants: Vec<ASTVariant<'a>>,
    pub elements: Vec<&'a str>,
    pub arbitrary: &'a str,
}

#[derive(Clone, Debug)]
pub struct ASTVariant<'a> {
    pub not: bool,
    pub pseudo: bool,
    pub names: Vec<&'a str>,
}
