use super::*;
use tailwind_ast::{parse_f32, parse_fraction, parse_integer, ASTVariant, AstStyle};

impl<'a> From<AstStyle<'a>> for TailwindInstruction {
    fn from(node: AstStyle<'a>) -> Self {
        Self {
            negative: node.negative,
            variants: node.variants.into_iter().map(|s| s.into()).collect(),
            elements: TailwindElements { inner: node.elements.into_iter().map(|s| s.to_string()).collect() },
            arbitrary: TailwindArbitrary { inner: node.arbitrary.map(|s| s.to_string()) },
        }
    }
}

impl<'a> From<ASTVariant<'a>> for TailwindVariant {
    fn from(node: ASTVariant<'a>) -> Self {
        Self { not: node.not, pseudo: node.pseudo, names: node.names.into_iter().map(|s| s.to_string()).collect() }
    }
}

impl TailwindInstruction {
    #[inline]
    pub fn view_elements(&self) -> Vec<&str> {
        self.elements.iter().map(|s| s.0.as_str()).collect()
    }
    #[inline]
    pub fn view_arbitrary(&self) -> &str {
        match &self.arbitrary {
            None => "",
            Some(setter) => setter.0.as_str(),
        }
    }
    // TODO
    pub fn normalization(self) -> Self {
        self
    }
}

impl TailwindArbitrary {
    #[inline]
    pub fn is_some(&self) -> bool {
        self.inner.is_some()
    }
    #[inline]
    pub fn is_none(&self) -> bool {
        self.inner.is_none()
    }
    #[inline]
    pub fn as_integer<T>(&self) -> T {
        let s = self.inner.unwrap_or_default();
        parse_integer(&s)?.1
    }
    #[inline]
    pub fn as_float(&self) -> f32 {
        let s = self.inner.unwrap_or_default();
        parse_f32(&s)?.1
    }
    #[inline]
    pub fn as_fraction(&self) -> (usize, usize) {
        let s = self.inner.unwrap_or_default();
        parse_fraction(&s)?.1
    }
}
