use peginator::PegParser;

use crate::{
    parser::tw::{GroupNode, InstructNode, TwParser, TwStatementNode},
    AstGroup, AstStyle,
};

mod tw;

#[test]
fn test() {
    let test = TwParser::parse("text(red bold)!\ntext-(red-bold!)!").unwrap();
    println!("{:#?}", test);
    println!("{:#?}", test.as_ast());
}

impl TwParser {
    pub fn as_ast(&self) -> Vec<AstGroup> {
        self.statements.iter().map(|s| s.as_ast()).collect()
    }
}

impl TwStatementNode {
    pub fn as_ast(&self) -> AstGroup {
        match self {
            TwStatementNode::GroupNode(v) => v.as_ast(),
            TwStatementNode::InstructNode(v) => {
                AstGroup { important: false, head: v.as_ast(), children: vec![] }
            }
        }
    }
}

impl GroupNode {
    pub fn as_ast(&self) -> AstGroup {
        AstGroup {
            important: false,
            head: AstStyle {
                important: false,
                negative: false,
                variants: vec![],
                elements: vec![],
                arbitrary: "".to_string(),
            },
            children: vec![],
        }
    }
}

impl InstructNode {
    pub fn as_ast(&self) -> AstStyle {
        AstStyle {
            important: self.important.is_some(),
            negative: false,
            variants: vec![],
            elements: vec![],
            arbitrary: "".to_string(),
        }
    }
}
