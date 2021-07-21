use crate::systems::instruction::{TailwindInstruction, TailwindVariant};

pub mod utils;
mod methods;
mod parse;

pub fn parse_tailwind(input: &str) -> Vec<TailwindInstruction> {}

///
#[derive(Debug)]
pub enum AstNode {
    Root(Vec<AstNode>),
    Styled(TailwindInstruction),
    /// `[.]`
    AstArbitrary(String),
    Grouped {
        variants: Vec<TailwindVariant>,
        elements: Option<AstElement>,
    },
}

pub struct AstStyle {}

pub struct ASTVariant<'a> {
    not: bool,
    pseudo: bool,
    names: Vec<&'a str>,
}

