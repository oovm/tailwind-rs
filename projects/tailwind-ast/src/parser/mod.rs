use std::collections::BTreeSet;

use peginator::{ParseError, PegParser};

use crate::{
    parser::tw::{
        ArbitraryItem, ArbitraryNode, ElementNode, GroupNode, InstructNode, StringItem, StringNode,
        TwParser, TwStatementNode, VariantItemNode, VariantNode,
    },
    ASTVariant, AstArbitrary, AstElements, AstStyle,
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
            TwStatementNode::ArbitraryNode(v) => v.as_ast(),
        }
    }
}

impl GroupNode {
    pub fn as_ast(&self) -> AstStyle {
        AstStyle {
            important: self.important.is_some(),
            variants: eat_variant(&self.variant),
            elements: self.element.as_ast(),
            arbitrary: AstArbitrary { item: "".to_string() },
            children: self.statements.iter().map(|s| s.as_ast()).collect(),
        }
    }
}

impl InstructNode {
    pub fn as_ast(&self) -> AstStyle {
        let variants = match &self.variant {
            Some(s) => s.as_ast(),
            None => Default::default(),
        };
        AstStyle {
            important: self.important.is_some(),
            variants,
            elements: self.element.as_ast(),
            arbitrary: AstArbitrary { item: "".to_string() },
            children: vec![],
        }
    }
}

impl ElementNode {
    pub fn as_ast(&self) -> AstElements {
        AstElements {
            negative: self.negative.is_some(),
            items: self.identifiers.iter().map(|f| f.to_string()).collect(),
        }
    }
}

impl VariantNode {
    pub fn as_ast(&self) -> BTreeSet<ASTVariant> {
        self.items.iter().map(|f| f.as_ast()).collect()
    }
}

impl VariantItemNode {
    pub fn as_ast(&self) -> ASTVariant {
        let mut out = ASTVariant {
            not: self.not.is_some(),
            pseudo: false,
            names: self.element.as_ast().items,
        };
        out.pseudo = match self.pseudo.as_str() {
            "::" => true,
            _ => check_pseudo(&out.as_view()),
        };
        out
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-elements#index
#[inline]
#[rustfmt::skip]
fn check_pseudo(names: &[&str]) -> bool {
    matches!(names
            , ["after"]
            | ["before"]
            | ["backdrop"]
            | ["marker"]
            | ["placeholder"]
            | ["selection"]
            | ["first", "line"]
            | ["first", "litter"]
            | ["first", "selector", "button"]
            | ["target", "text"]
        )
}

impl ArbitraryNode {
    pub fn as_ast(&self) -> AstStyle {
        AstStyle {
            important: self.important.is_some(),
            variants: eat_variant(&self.variant),
            elements: self.element.as_ast(),
            arbitrary: self.item.as_ast(),
            children: vec![],
        }
    }
}
impl ArbitraryItem {
    pub fn as_ast(&self) -> AstArbitrary {
        let item = match self {
            ArbitraryItem::ArbitraryBalance(v) => v.to_string(),
            ArbitraryItem::StringNode(v) => v.as_ast(),
        };
        AstArbitrary { item }
    }
}

impl StringNode {
    pub fn as_ast(&self) -> String {
        let mut out = String::new();
        for x in &self.item {
            out.push(x.as_ast())
        }
        out
    }
}

impl StringItem {
    pub fn as_ast(&self) -> char {
        match self {
            StringItem::Any(c) => *c,
            StringItem::Escaped(c) => match c.any {
                'r' => '\r',
                'n' => '\n',
                _ => c.any,
            },
        }
    }
}

fn eat_variant(variant: &Option<VariantNode>) -> BTreeSet<ASTVariant> {
    match variant {
        Some(s) => s.as_ast(),
        None => Default::default(),
    }
}
