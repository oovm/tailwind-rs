use serde::{Deserialize, Serialize};

/// Versioned, data-driven contributions (built-in or plugin).
///
/// **Status (honest):** only `id` + `protocol_version` are frozen on the wire today.
/// Matcher / value schema / resolver ID / variant effect / order / conflict policy /
/// capability negotiation are **not** implemented. Non-empty
/// [`ContributionSet`] must produce `contribution_unsupported` diagnostics —
/// the engine must not silently ignore them.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributionSet {
    pub items: Vec<Contribution>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contribution {
    pub id: String,
    pub protocol_version: u32,
}
