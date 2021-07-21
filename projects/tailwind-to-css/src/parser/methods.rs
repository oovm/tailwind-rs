use super::*;


impl<'a> From<ASTVariant<'a>> for TailwindVariant {
    fn from(_: ASTVariant<'a>) -> Self {
        Self {
            not: false,
            pseudo: false,
            names: vec![]
        }
    }
}