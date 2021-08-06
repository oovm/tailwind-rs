use super::*;

impl Default for CssAttributes {
    fn default() -> Self {
        Self {
            apply: None,
            normal: Default::default(),
            transforms: Default::default(),
            scoped: Default::default(),
            ring_resolver: None,
        }
    }
}

impl CssAttributes {
    pub fn insert<S>(&mut self, key: S, value: S)
    where
        S: Into<String>,
    {
        self.normal.insert(key.into(), value.into());
    }
    pub fn transform<S>(&mut self, key: S, value: S)
    where
        S: Into<String>,
    {
        self.transforms.insert(key.into(), value.into());
    }
}
