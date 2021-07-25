use super::*;

impl<'a> From<ASTVariant<'a>> for TailwindVariant {
    fn from(node: ASTVariant<'a>) -> Self {
        Self::new(&node.names, node.not, node.pseudo)
    }
}
