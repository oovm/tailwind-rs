use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineStats {
    pub candidate_count: u32,
    pub rule_count: u32,
    pub diagnostic_count: u32,
    /// Parse-layer cache hits (only when `EngineOptions.collect_stats`).
    #[serde(default)]
    pub parse_cache_hits: u32,
    /// Resolve-layer cache hits (only when `collect_stats`).
    #[serde(default)]
    pub resolve_cache_hits: u32,
    /// Module-layer cache hits (only when `collect_stats`).
    #[serde(default)]
    pub module_cache_hits: u32,
}
