use super::*;

mod traits;

/// A css property is used to remove duplicates.
///
/// In principle, each css property will only appear once, and the one set later will override the previous one.
#[derive(Clone)]
pub struct CssAttribute {
    key: String,
    value: String,
}

impl CssAttribute {
    /// # Arguments
    ///
    /// * `key`: css class
    /// * `value`: css property
    ///
    /// returns: [`CssAttribute`]
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Self { key: key.into(), value: value.into() }
    }
}
