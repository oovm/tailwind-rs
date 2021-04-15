use crate::{DiagnosticLevel, JssError, JssErrorKind};
use html_parser::Error;

impl From<Error> for JssError {
    fn from(e: Error) -> Self {
        let kind = match e {
            Error::Parsing(e) => JssErrorKind::SyntaxError(e),
            Error::IO(e) => JssErrorKind::IOError(e),
            Error::Cli(_) => {
                unimplemented!()
            }
            Error::Serde(_) => {
                unimplemented!()
            }
        };
        Self { kind: Box::new(kind), level: DiagnosticLevel::None, file: None, range: None }
    }
}
