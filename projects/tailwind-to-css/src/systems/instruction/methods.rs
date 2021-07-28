use super::*;
use tailwind_ast::{parse_f32, parse_fraction, parse_integer, ASTVariant};

impl From<&str> for TailwindArbitrary {
    fn from(node: &str) -> Self {
        Self {
            inner: match node.is_empty() {
                true => Some(node.to_string()),
                false => None,
            },
        }
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

impl<'a> From<ASTVariant<'a>> for TailwindVariant {
    fn from(node: ASTVariant<'a>) -> Self {
        Self::new(&node.names, node.not, node.pseudo)
    }
}
