use peginator::{ParseError, PegParser};

use crate::{
    parser::tw::{ElementNode, GroupNode, InstructNode, TwParser, TwStatementNode},
    AstArbitrary, AstElements, AstStyle,
};

mod tw;

/// Parse tailwind text to groups
pub fn parse(input: &str) -> Result<Vec<AstStyle>, ParseError> {
    let raw = TwParser::parse(input)?.statements;
    Ok(raw.into_iter().map(|s| s.as_ast()).collect())
}

impl TwStatementNode {
    pub fn as_ast(&self) -> AstStyle {
        match self {
            TwStatementNode::GroupNode(v) => v.as_ast(),
            TwStatementNode::InstructNode(v) => v.as_ast(),
        }
    }
}

impl GroupNode {
    pub fn as_ast(&self) -> AstStyle {
        AstStyle {
            important: self.important.is_some(),
            negative: false,
            variants: vec![],
            elements: self.element.as_ast(),
            arbitrary: AstArbitrary { item: "".to_string() },
            children: self.statements.iter().map(|s| s.as_ast()).collect(),
        }
    }
}

impl InstructNode {
    pub fn as_ast(&self) -> AstStyle {
        AstStyle {
            important: self.important.is_some(),
            negative: false,
            variants: vec![],
            elements: self.element.as_ast(),
            arbitrary: AstArbitrary { item: "".to_string() },
            children: vec![],
        }
    }
}

impl ElementNode {
    pub fn as_ast(&self) -> AstElements {
        AstElements { items: self.identifiers.iter().map(|f| f.to_string()).collect() }
    }
}
