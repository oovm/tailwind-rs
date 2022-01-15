use super::*;

mod traits;

/// A css property is used to remove duplicates.
///
/// In principle, each css property will only appear once, and the one set later will override the previous one.
#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CssAttributes {
    normal: BTreeMap<String, String>,
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
        let forbid = BTreeSet::from_iter(vec!["transform", "backdrop-filter", "filter"]);
        if forbid.contains(key.as_str()) {
            panic!("can't use insert on {}", key);
        }
        self.normal.insert(key, value.into());
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
        self.normal.extend(items)
    }

    /// # Arguments
    ///
    /// * `value`:
    ///
    /// returns: ()
    pub fn transform<V>(&mut self, value: V)
    where
        V: Into<String>,
    {
        self.transforms.insert(value.into());
    }
    /// # Arguments
    ///
    /// * `value`:
    ///
    /// returns: ()
    pub fn filter<V>(&mut self, value: V)
    where
        V: Into<String>,
    {
        self.filter.insert(value.into());
    }
    /// # Arguments
    ///
    /// * `value`:
    ///
    /// returns: ()
    pub fn backdrop_filter<V>(&mut self, value: V)
    where
        V: Into<String>,
    {
        self.backdrop_filter.insert(value.into());
    }
}
