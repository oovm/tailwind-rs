use super::*;
mod methods;

#[derive(Debug, Clone)]
pub struct TailwindArbitrary {
    inner: String,
}

impl From<&str> for TailwindArbitrary {
    fn from(s: &str) -> Self {
        Self { inner: s.to_string() }
    }
}

impl Display for TailwindArbitrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner.is_empty() {
            true => {
                write!(f, "")
            },
            false => {
                write!(f, "-[{}]", self.inner)
            },
        }
    }
}
