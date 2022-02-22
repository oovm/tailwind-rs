use super::*;

impl ImportantSet {
    pub fn insert<T>(&mut self, value: T) -> bool
    where
        T: Into<String>,
    {
        self.important = false;
        self.set.insert(value.into())
    }
    pub fn insert_important<T>(&mut self, value: T) -> bool
    where
        T: Into<String>,
    {
        self.important = true;
        self.set.insert(value.into())
    }
}

impl ImportantMap {
    pub fn insert<K, V>(&mut self, key: K, value: V) -> bool
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.map.insert(key.into(), (false, value.into())).is_some()
    }
    pub fn insert_important<K, V>(&mut self, key: K, value: V) -> bool
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.map.insert(key.into(), (true, value.into())).is_some()
    }
}
