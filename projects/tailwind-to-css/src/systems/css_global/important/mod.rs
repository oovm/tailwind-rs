use super::*;

mod methods;
mod traits;

/// The `css-global-attribute` system.
#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImportantSet {
    important: bool,
    set: BTreeSet<String>,
}

/// The `css-global-attribute` system.
#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImportantMap {
    map: BTreeMap<String, (bool, String)>,
}
