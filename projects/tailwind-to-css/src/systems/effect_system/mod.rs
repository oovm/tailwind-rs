use std::collections::BTreeMap;

mod builtin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct EffectSystem {
    box_shadow_default: String,
    box_shadows: BTreeMap<String, String>,
    drop_shadow_default: String,
    drop_shadows: BTreeMap<String, String>,
}

impl EffectSystem {
    pub fn get_box_shadow(&self, name: &str) -> String {
        self.box_shadows.get(name).unwrap_or(&self.box_shadow_default).clone()
    }
    pub fn insert_box_shadow<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.box_shadows.insert(key.into(), value.into());
    }
    pub fn set_box_shadow_default<V>(&mut self, value: V)
    where
        V: Into<String>,
    {
        self.box_shadow_default = value.into();
    }
    pub fn get_drop_shadow(&self, name: &str) -> String {
        self.box_shadows.get(name).unwrap_or(&self.drop_shadow_default).clone()
    }
    pub fn insert_drop_shadow<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.drop_shadows.insert(key.into(), value.into());
    }
    pub fn set_drop_shadow_default<V>(&mut self, value: V)
    where
        V: Into<String>,
    {
        self.drop_shadow_default = value.into();
    }
}
