use super::*;

impl From<bool> for Negative {
    fn from(n: bool) -> Self {
        Self(n)
    }
}

impl PartialEq<bool> for Negative {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}
