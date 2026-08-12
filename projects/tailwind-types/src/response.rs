use serde::{Deserialize, Serialize};

use super::{CanonicalStyleModule, Diagnostic, EngineStats};

/// Sole public compile output.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileResponse {
    pub module: CanonicalStyleModule,
    pub diagnostics: Vec<Diagnostic>,
    pub stats: EngineStats,
}
