use serde::{Deserialize, Serialize};

use super::SourceRef;

/// Origin of a canonical rule. Engine treats `SourceRef` as opaque.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    pub source: SourceRef,
    /// Index of the candidate inside `CompileRequest.candidates`, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidate_index: Option<u32>,
}
