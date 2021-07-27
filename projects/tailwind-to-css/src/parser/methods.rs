use super::*;

impl<'a> From<ASTVariant<'a>> for TailwindVariant {
    fn from(node: ASTVariant<'a>) -> Self {
        Self::new(&node.names, node.not, node.pseudo)
    }
}

impl<'a> From<AstArbitrary<'a>> for TailwindArbitrary {
    fn from(node: AstArbitrary<'a>) -> Self {
        let s = node.0;
        Self {
            inner: match node.0.is_empty() {
                true => Some(name.to_string()),
                false => None,
            },
        }
    }
}
