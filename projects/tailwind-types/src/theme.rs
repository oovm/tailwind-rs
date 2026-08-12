use serde::{Deserialize, Serialize};

use super::ThemeValue;

/// Theme namespaces accepted by a compile request.
///
/// Values are normalized into `ThemeSnapshot` during resolve (C2).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeInput {
    pub entries: Vec<ThemeEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeEntry {
    pub key: ThemeKey,
    pub value: ThemeValue,
}

/// Stable theme lookup key, e.g. `colors.red.500` or `spacing.4`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ThemeKey {
    pub path: Vec<String>,
}

impl ThemeKey {
    pub fn from_path<I, S>(parts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            path: parts.into_iter().map(Into::into).collect(),
        }
    }

    pub fn joined(&self) -> String {
        self.path.join(".")
    }
}
