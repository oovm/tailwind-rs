use serde::{Deserialize, Serialize};

use super::{ConditionTree, Declaration, OrderKey, Provenance, ThemeKey};

/// Deterministic, structured compile result.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalStyleModule {
    pub rules: Vec<CanonicalRule>,
    pub referenced_theme_keys: Vec<ThemeKey>,
    /// Stable content hash of the canonical rules (FNV-1a, process-independent).
    pub content_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalRule {
    pub candidate_key: String,
    pub conditions: ConditionTree,
    pub declarations: Vec<Declaration>,
    pub order: OrderKey,
    pub provenance: Vec<Provenance>,
}
