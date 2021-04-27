use super::*;


impl CssAttribute {
    pub fn new<S: Into<String>>(key: S, value: S) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Display for CssAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {};", self.key, self.value)
    }
}