use super::*;

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

impl From<String> for TailwindArbitrary {
    fn from(s: String) -> Self {
        Self { inner: Box::from(s) }
    }
}
impl From<&String> for TailwindArbitrary {
    fn from(s: &String) -> Self {
        Self { inner: Box::from(s.as_str()) }
    }
}