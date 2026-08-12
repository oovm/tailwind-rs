use serde::{Deserialize, Serialize};

/// Deterministic ordering key. Must not depend on HashMap iteration,
/// registration order, or input file order.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OrderKey {
    /// Layer / cascade band (preflight, base, components, utilities, …).
    pub layer: u16,
    /// Variant composition rank.
    pub variant_rank: u32,
    /// Utility family rank inside the registry.
    pub utility_rank: u32,
    /// Stable tie-breaker (usually normalized candidate key).
    pub tie: String,
}
