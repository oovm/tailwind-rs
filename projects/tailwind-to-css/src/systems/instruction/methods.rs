use super::*;
use css_color::Srgb;
use tailwind_ast::{parse_f32, parse_fraction, parse_integer, ASTVariant, AstStyle};

impl<'a> From<AstStyle<'a>> for TailwindInstruction {
    fn from(node: AstStyle<'a>) -> Self {
        Self {
            negative: node.negative,
            variants: node.variants.into_iter().map(|s| s.into()).collect(),
            elements: TailwindElements { inner: node.elements.into_iter().map(|s| s.to_string()).collect() },
            arbitrary: TailwindArbitrary { inner: node.arbitrary.unwrap_or_default().to_string() },
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
        Self { inner: s.to_string() }
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
        self.inner.is_empty()
    }
    #[inline]
    pub fn is_none(&self) -> bool {
        !self.inner.is_empty()
    }
    #[inline]
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
    #[inline]
    pub fn as_integer<T>(&self) -> Result<T>
    where
        T: FromStr,
    {
        Ok(parse_integer(&self.inner)?.1)
    }
    #[inline]
    pub fn as_float(&self) -> Result<f32> {
        Ok(parse_f32(&self.inner)?.1)
    }
    #[inline]
    pub fn as_fraction(&self) -> Result<(usize, usize)> {
        Ok(parse_fraction(&self.inner)?.1)
    }
    #[inline]
    pub fn as_length(&self) -> Result<LengthUnit> {
        Ok(LengthUnit::parse(&self.inner)?.1)
    }
    #[inline]
    pub fn as_color(&self) -> Result<Srgb> {
        Ok(Srgb::from_str(&self.inner)?)
    }
}
