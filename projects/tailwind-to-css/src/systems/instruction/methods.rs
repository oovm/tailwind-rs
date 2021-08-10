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

impl From<&str> for TailwindArbitrary {
    fn from(s: &str) -> Self {
        Self {
            inner: match s.is_empty() {
                true => None,
                false => Some(s.to_string()),
            },
        }
    }
}

impl TailwindInstruction {
    #[inline]
    pub fn view_elements(&self) -> Vec<&str> {
        self.elements.inner.iter().map(|s| s.as_str()).collect()
    }
    #[inline]
    pub fn view_arbitrary(&self) -> &TailwindArbitrary {
        &self.arbitrary
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
    pub fn as_str(&self) -> &str {
        match &self.inner {
            None => "",
            Some(s) => s.as_str(),
        }
    }
    #[inline]
    pub fn as_integer<T>(&self) -> Result<T>
    where
        T: FromStr,
    {
        match &self.inner {
            None => syntax_error!("missing arbitrary"),
            Some(s) => Ok(parse_integer(s)?.1),
        }
    }
    #[inline]
    pub fn as_float(&self) -> Result<f32> {
        match &self.inner {
            None => syntax_error!("missing arbitrary"),
            Some(s) => Ok(parse_f32(s)?.1),
        }
    }
    #[inline]
    pub fn as_fraction(&self) -> Result<(usize, usize)> {
        match &self.inner {
            None => syntax_error!("missing arbitrary"),
            Some(s) => Ok(parse_fraction(s)?.1),
        }
    }
    #[inline]
    pub fn as_length(&self) -> Result<LengthUnit> {
        match &self.inner {
            None => syntax_error!("missing arbitrary"),
            Some(s) => Ok(LengthUnit::parse(s)?.1),
        }
    }
}
