use std::collections::btree_map::Iter;

use super::*;

mod traits;

/// A css property is used to remove duplicates.
///
/// In principle, each css property will only appear once, and the one set later will override the previous one.
#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CssAttributes {
    normal: BTreeMap<String, String>,
    transforms: BTreeSet<String>,
}

impl CssAttributes {
    /// # Arguments
    ///
    /// * `key`: css class
    /// * `value`: css property
    ///
    /// returns: [`CssAttribute`]
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.normal.insert(key.into(), value.into());
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
    pub fn iter(&self) -> Iter<'_, String, String> {
        self.normal.iter()
    }
}
