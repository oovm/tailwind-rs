use super::*;

impl TailwindArbitrary {
    #[inline]
    pub fn is_some(&self) -> bool {
        self.inner.is_some()
    }
    #[inline]
    pub fn is_none(&self) -> bool {
        self.inner.is_none()
    }

    pub fn as_u<T>(&self) -> T {
        let s = self.inner.unwrap_or_default();

        parse_integer(s)
    }
    pub fn as_i(&self) -> usize {
        todo!()
    }
}
