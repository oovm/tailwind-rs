use crate::systems::instruction::{TailwindArbitrary, TailwindInstruction, TailwindVariant};
mod methods;
mod parse;
pub mod utils;
// mod tests;
use crate::TailwindBuilder;
use tailwind_error::nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};

pub fn parse_tailwind(input: &str) -> Vec<TailwindInstruction> {
    todo!()
}

///
#[derive(Debug)]
pub(crate) enum AstNode<'a> {
    Root(Vec<AstNode<'a>>),
    Styled(TailwindInstruction),
    /// `[.]`
    AstArbitrary(AstArbitrary<'a>),
    Grouped {
        variants: Vec<ASTVariant<'a>>,
        elements: Option<AstElement>,
        children: Vec<AstNode<'a>>,
    },
}

struct AstStyle {}

struct AstArbitrary<'a>(&'a str);

struct ASTVariant<'a> {
    not: bool,
    pseudo: bool,
    names: Vec<&'a str>,
}
