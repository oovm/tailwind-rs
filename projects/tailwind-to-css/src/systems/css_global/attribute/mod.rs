use super::*;
mod traits;

///
#[derive(Clone)]
pub struct CssAttribute {
    key: String,
    value: String,
}

impl CssAttribute {
    pub fn new<K>(key: K, value: K) -> Self
    where
        K: Into<String>,
    {
        Self { key: key.into(), value: value.into() }
    }
}
