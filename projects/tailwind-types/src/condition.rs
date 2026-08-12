use serde::{Deserialize, Serialize};

/// Normalized condition tree wrapping declarations.
///
/// Built from variant effects during resolve/normalize. Not a CSS string.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ConditionTree {
    /// No media / selector / support wrappers.
    None,
    Selector(SelectorCondition),
    AtRule(AtRuleCondition),
    Compound(Vec<ConditionTree>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SelectorCondition {
    /// Pure selector transform description, e.g. `:hover`, `.dark &`.
    pub transform: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AtRuleCondition {
    /// e.g. `media`, `supports`, `container`
    pub name: String,
    pub query: String,
}
