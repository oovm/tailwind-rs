use crate::{DiagnosticLevel, TailwindError, TailwindErrorKind};
use html_parser::Error;

impl From<Error> for TailwindError {
    fn from(e: Error) -> Self {
        let kind = match e {
            Error::Parsing(e) => TailwindErrorKind::SyntaxError(e),
            Error::IO(e) => TailwindErrorKind::IOError(e),
            Error::Cli(_) => {
                unimplemented!()
            },
            Error::Serde(_) => {
                unimplemented!()
            },
        };
        Self { kind: Box::new(kind), level: DiagnosticLevel::None, file: None, range: None }
    }
}
