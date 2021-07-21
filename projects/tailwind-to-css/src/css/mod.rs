use crate::AstStyle;
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

mod attribute;

pub struct CssApplyResolver {
    apply: BTreeMap<String, CssAttributes>,
}

pub struct CssAttributes {
    apply: Option<String>,
    normal: BTreeMap<String, String>,
    transforms: BTreeMap<String, String>,
    scoped: BTreeMap<String, CssAttributes>,
    tw_border_opacity: bool,
}

impl Default for CssAttributes {
    fn default() -> Self {
        Self { apply: None, normal: Default::default(), transforms: Default::default(), scoped: Default::default() }
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

impl Display for CssAttributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
