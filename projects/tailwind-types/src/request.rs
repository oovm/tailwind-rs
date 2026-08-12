use serde::{Deserialize, Serialize};

use super::{CandidateInput, ContributionSet, EngineOptions, ThemeInput};

/// Sole public compile input.
///
/// Must not grow fields for file paths, HTML, DOM, project roots, VMZ, or Doki.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileRequest {
    pub candidates: Vec<CandidateInput>,
    #[serde(default)]
    pub theme: ThemeInput,
    #[serde(default)]
    pub contributions: ContributionSet,
    #[serde(default)]
    pub options: EngineOptions,
}
