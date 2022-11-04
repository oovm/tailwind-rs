use peginator::PegParser;

use crate::{
    parser::tw::{GroupNode, InstructNode, TwParser, TwStatementNode},
    AstArbitrary, AstStyle,
};

mod tw;

#[test]
fn test() {
    let test = TwParser::parse("text(red bold)!\ntext-(red-bold!)!").unwrap();
    println!("{:#?}", test);
    for x in test.as_ast() {
        println!("{}", x);
    }
}

impl TwParser {
    pub fn as_ast(&self) -> Vec<AstStyle> {
        self.statements.iter().map(|s| s.as_ast()).collect()
    }
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
            elements: vec![],
            arbitrary: AstArbitrary { arbitrary: "".to_string() },
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
            arbitrary: AstArbitrary { arbitrary: "".to_string() },
            children: vec![],
        }
    }
}
