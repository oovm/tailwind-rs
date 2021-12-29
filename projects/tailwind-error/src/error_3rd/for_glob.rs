use crate::TailwindError;
use glob::{GlobError, PatternError};

impl From<PatternError> for TailwindError {
    fn from(e: PatternError) -> Self {
        TailwindError::runtime_error(e.to_string())
    }
}

impl From<GlobError> for TailwindError {
    fn from(e: GlobError) -> Self {
        TailwindError::runtime_error(e.to_string())
    }
}
