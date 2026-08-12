use serde::{Deserialize, Serialize};

/// Engine knobs that do not change rule semantics of a given registry/theme.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineOptions {
    /// When true, emit verbose resolve stats.
    #[serde(default)]
    pub collect_stats: bool,
}
