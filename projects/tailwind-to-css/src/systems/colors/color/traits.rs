use super::*;

impl From<Srgb> for TailwindColor {
    fn from(c: Srgb) -> Self {
        Self::RGB(c)
    }
}

impl From<&str> for TailwindColor {
    fn from(s: &str) -> Self {
        Self::Keyword(s.to_string())
    }
}

impl From<String> for TailwindColor {
    fn from(s: String) -> Self {
        Self::Keyword(s)
    }
}
