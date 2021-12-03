use super::*;

impl TailwindArbitrary {
    #[inline]
    pub fn is_some(&self) -> bool {
        !self.inner.is_empty()
    }
    #[inline]
    pub fn is_none(&self) -> bool {
        self.inner.is_empty()
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
        LengthUnit::parse_length(&self.inner)
    }
    #[inline]
    pub fn as_color(&self) -> Result<Srgb> {
        Ok(Srgb::from_str(&self.inner)?)
    }
}
