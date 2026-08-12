use serde::{Deserialize, Serialize};

/// Opaque host-provided source identity.
///
/// The engine stores and returns this in provenance only. It must not interpret
/// the string as a path, URL, VMZ span, Doki node, or any other host concept.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceRef(pub String);

impl SourceRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One candidate token handed to the engine.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateInput {
    /// Raw candidate text, e.g. `hover:bg-red-500` or `p-[1.5rem]`.
    pub token: String,
    /// Opaque host identity for provenance round-trip.
    pub source: SourceRef,
    /// Optional stable key chosen by the caller for caching / identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable_key: Option<String>,
}

impl CandidateInput {
    pub fn new(token: impl Into<String>, source: SourceRef) -> Self {
        Self {
            token: token.into(),
            source,
            stable_key: None,
        }
    }

    pub fn with_stable_key(mut self, key: impl Into<String>) -> Self {
        self.stable_key = Some(key.into());
        self
    }
}
