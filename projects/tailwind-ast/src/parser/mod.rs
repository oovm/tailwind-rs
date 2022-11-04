use peginator::PegParser;

use crate::parser::tw::{GroupNode, InstructNode, TwParser, TwStatementNode};

mod tw;

#[test]
fn test() {
    let test = TwParser::parse("test-red").unwrap();
    println!("{:#?}", test);
    println!("{:#?}", test.as_ast());
}

impl TwParser {
    pub fn as_ast(&self) {
        for x in &self.statements {
            x.as_ast()
        }
    }
}

impl TwStatementNode {
    pub fn as_ast(&self) {
        match self {
            TwStatementNode::GroupNode(v) => v.as_ast(),
            TwStatementNode::InstructNode(v) => v.as_ast(),
        }
    }
}

impl GroupNode {
    pub fn as_ast(&self) {}
}

impl InstructNode {
    pub fn as_ast(&self) {}
}
