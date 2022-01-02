use super::*;
use std::fmt::Write;

mod methods;

#[derive(Debug, Clone)]
pub struct TailwindArbitrary {
    inner: Box<str>,
}

impl Display for TailwindArbitrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for c in self.inner.chars() {
            match c {
                ' ' => f.write_char('_')?,
                _ => f.write_char(c)?,
            }
        }
        f.write_char(']')
    }
}

impl From<&str> for TailwindArbitrary {
    fn from(s: &str) -> Self {
        Self { inner: Box::from(s) }
    }
}

impl From<&Self> for TailwindArbitrary {
    fn from(s: &Self) -> Self {
        Self { inner: s.inner.clone() }
    }
}

impl TailwindArbitrary {
    pub fn new<T>(s: T) -> Result<Self>
    where
        T: Into<Self>,
    {
        let out = s.into();
        if cfg!(compile_time) {
            if out.inner.is_empty() {
                return Err(TailwindError::syntax_error("Arbitrary value cannot be empty"));
            }
            // TODO: Check unbalanced quotes
            if out.inner.contains('\n') {
                return Err(TailwindError::syntax_error("Arbitrary value does balance quotes"));
            }
        }
        Ok(out)
    }

    pub fn get_class(&self) -> String {
        let mut class = String::with_capacity(self.inner.len() + 2);
        class.push('[');
        for c in self.inner.chars() {
            match c {
                ' ' => class.push('_'),
                _ => class.push(c),
            }
        }
        class.push(']');
        class
    }
    pub fn write(&self, f: &mut Formatter) -> std::fmt::Result {
        self.write_class(f, "")
    }
    pub fn write_class(&self, f: &mut Formatter, before: &str) -> std::fmt::Result {
        write!(f, "{}{}", before, self.get_class())
    }
    pub fn get_properties(&self) -> String {
        self.inner.to_string()
    }
}
