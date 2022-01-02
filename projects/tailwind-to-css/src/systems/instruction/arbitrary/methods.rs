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
        self.inner.as_ref()
    }
    #[inline]
    pub fn as_integer(&self) -> Result<i32> {
        Ok(i32::from_str(&self.inner)?)
    }
    #[inline]
    pub fn as_float(&self) -> Result<f32> {
        Ok(f32::from_str(&self.inner)?)
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
    pub fn as_length_or_fraction(&self) -> Result<LengthUnit> {
        LengthUnit::parse_length(&self.inner).or_else(|_| LengthUnit::parse_faction(&self.inner))
    }
    #[inline]
    pub fn as_angle(&self) -> Result<LengthUnit> {
        LengthUnit::parse_angle(&self.inner)
    }
    #[inline]
    pub fn as_color(&self) -> Result<Srgb> {
        Ok(Srgb::from_str(&self.inner)?)
    }
}
