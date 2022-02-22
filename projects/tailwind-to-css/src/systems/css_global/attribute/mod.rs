use super::*;

mod traits;

/// A css property is used to remove duplicates.
///
/// In principle, each css property will only appear once, and the one set later will override the previous one.
#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CssAttributes {
    normal: ImportantMap<String, String>,
    transforms: BTreeSet<String>,
    backdrop_filter: BTreeSet<String>,
    filter: BTreeSet<String>,
}

impl CssAttributes {
    /// # Arguments
    ///
    /// * `key`: css class
    /// * `value`: css property
    ///
    /// returns: [`CssAttribute`]
    #[track_caller]
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        match key.as_str() {
            "transform" => self.transforms.insert(value.into()),
            "backdrop-filter" => self.backdrop_filter.insert(value.into()),
            "filter" => self.filter.insert(value.into()),
            _ => self.normal.insert(key, value.into()).is_some(),
        };
    }

    /// # Arguments
    ///
    /// * `items`:
    ///
    /// returns: ()
    pub fn extend<T>(&mut self, items: T)
    where
        T: IntoIterator<Item = (String, String)>,
    {
        for i in items {
            self.insert(i.0, i.1);
        }
    }
}
